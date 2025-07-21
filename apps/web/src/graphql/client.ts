// GraphQL Client for Free Deep Research System
// Phase 4.4: API Gateway & GraphQL

import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  split,
  from,
  ApolloLink,
} from '@apollo/client';
import { getMainDefinition } from '@apollo/client/utilities';
import { GraphQLWsLink } from '@apollo/client/link/subscriptions';
import { createClient } from 'graphql-ws';
import { setContext } from '@apollo/client/link/context';
import { onError } from '@apollo/client/link/error';
import { RetryLink } from '@apollo/client/link/retry';

// GraphQL endpoint configuration
const GRAPHQL_HTTP_URI = process.env.NODE_ENV === 'production' 
  ? 'https://graphql.freedeepresearch.org/graphql'
  : 'http://localhost:4000/graphql';

const GRAPHQL_WS_URI = process.env.NODE_ENV === 'production'
  ? 'wss://graphql.freedeepresearch.org/graphql'
  : 'ws://localhost:4000/graphql';

// HTTP Link for queries and mutations
const httpLink = createHttpLink({
  uri: GRAPHQL_HTTP_URI,
  credentials: 'include',
});

// WebSocket Link for subscriptions
const wsLink = new GraphQLWsLink(
  createClient({
    url: GRAPHQL_WS_URI,
    connectionParams: () => {
      const token = localStorage.getItem('authToken');
      return {
        authorization: token ? `Bearer ${token}` : '',
      };
    },
    retryAttempts: 5,
    shouldRetry: () => true,
  })
);

// Auth Link to add authentication headers
const authLink = setContext((_, { headers }) => {
  const token = localStorage.getItem('authToken');
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : '',
      'x-client-name': 'free-deep-research-web',
      'x-client-version': '4.4.0',
    },
  };
});

// Error Link for handling GraphQL errors
const errorLink = onError(({ graphQLErrors, networkError, operation, forward }) => {
  if (graphQLErrors) {
    graphQLErrors.forEach(({ message, locations, path, extensions }) => {
      console.error(
        `GraphQL error: Message: ${message}, Location: ${locations}, Path: ${path}`
      );
      
      // Handle authentication errors
      if (extensions?.code === 'UNAUTHENTICATED') {
        localStorage.removeItem('authToken');
        window.location.href = '/login';
      }
      
      // Handle authorization errors
      if (extensions?.code === 'FORBIDDEN') {
        console.error('Access denied:', message);
        // Show user-friendly error message
      }
      
      // Handle rate limiting
      if (extensions?.code === 'RATE_LIMITED') {
        console.warn('Rate limit exceeded:', message);
        // Show rate limit warning to user
      }
    });
  }

  if (networkError) {
    console.error(`Network error: ${networkError}`);
    
    // Handle network errors
    if (networkError.statusCode === 401) {
      localStorage.removeItem('authToken');
      window.location.href = '/login';
    }
  }
});

// Retry Link for handling temporary failures
const retryLink = new RetryLink({
  delay: {
    initial: 300,
    max: Infinity,
    jitter: true,
  },
  attempts: {
    max: 3,
    retryIf: (error, _operation) => !!error,
  },
});

// Split link to route queries/mutations to HTTP and subscriptions to WebSocket
const splitLink = split(
  ({ query }) => {
    const definition = getMainDefinition(query);
    return (
      definition.kind === 'OperationDefinition' &&
      definition.operation === 'subscription'
    );
  },
  wsLink,
  from([authLink, errorLink, retryLink, httpLink])
);

// Apollo Client cache configuration
const cache = new InMemoryCache({
  typePolicies: {
    Query: {
      fields: {
        // Pagination for users
        users: {
          keyArgs: ['filter'],
          merge(existing, incoming) {
            if (!existing) return incoming;
            return {
              ...incoming,
              edges: [...(existing.edges || []), ...(incoming.edges || [])],
            };
          },
        },
        
        // Pagination for workflows
        researchWorkflows: {
          keyArgs: ['filter'],
          merge(existing, incoming) {
            if (!existing) return incoming;
            return {
              ...incoming,
              edges: [...(existing.edges || []), ...(incoming.edges || [])],
            };
          },
        },
        
        // Pagination for API keys
        apiKeys: {
          keyArgs: ['filter'],
          merge(existing, incoming) {
            if (!existing) return incoming;
            return {
              ...incoming,
              edges: [...(existing.edges || []), ...(incoming.edges || [])],
            };
          },
        },
      },
    },
    
    // Cache configuration for specific types
    User: {
      fields: {
        preferences: {
          merge: true,
        },
      },
    },
    
    ResearchWorkflow: {
      fields: {
        configuration: {
          merge: true,
        },
        collaborators: {
          merge: false,
        },
      },
    },
    
    SystemMetrics: {
      keyFields: ['timestamp'],
    },
    
    WorkflowExecution: {
      fields: {
        results: {
          merge: true,
        },
      },
    },
  },
});

// Create Apollo Client instance
export const apolloClient = new ApolloClient({
  link: splitLink,
  cache,
  defaultOptions: {
    watchQuery: {
      errorPolicy: 'all',
      notifyOnNetworkStatusChange: true,
    },
    query: {
      errorPolicy: 'all',
    },
    mutate: {
      errorPolicy: 'all',
    },
  },
  connectToDevTools: process.env.NODE_ENV === 'development',
});

// GraphQL operation helpers
export const graphqlHelpers = {
  // Execute a query with error handling
  async query<T = any>(query: any, variables?: any): Promise<T> {
    try {
      const result = await apolloClient.query({
        query,
        variables,
        fetchPolicy: 'cache-first',
      });
      return result.data;
    } catch (error) {
      console.error('GraphQL query error:', error);
      throw error;
    }
  },

  // Execute a mutation with error handling
  async mutate<T = any>(mutation: any, variables?: any): Promise<T> {
    try {
      const result = await apolloClient.mutate({
        mutation,
        variables,
        errorPolicy: 'all',
      });
      return result.data;
    } catch (error) {
      console.error('GraphQL mutation error:', error);
      throw error;
    }
  },

  // Subscribe to real-time updates
  subscribe(subscription: any, variables?: any) {
    return apolloClient.subscribe({
      query: subscription,
      variables,
      errorPolicy: 'all',
    });
  },

  // Clear cache
  clearCache() {
    return apolloClient.clearStore();
  },

  // Refetch all active queries
  refetchQueries() {
    return apolloClient.refetchQueries({
      include: 'active',
    });
  },
};

// Authentication helpers
export const authHelpers = {
  // Set authentication token
  setAuthToken(token: string) {
    localStorage.setItem('authToken', token);
    // Reset Apollo Client to clear any cached data
    apolloClient.resetStore();
  },

  // Remove authentication token
  removeAuthToken() {
    localStorage.removeItem('authToken');
    // Clear Apollo Client cache
    apolloClient.clearStore();
  },

  // Get current authentication token
  getAuthToken(): string | null {
    return localStorage.getItem('authToken');
  },

  // Check if user is authenticated
  isAuthenticated(): boolean {
    return !!this.getAuthToken();
  },
};

// Cache helpers
export const cacheHelpers = {
  // Update user in cache
  updateUser(user: any) {
    apolloClient.cache.writeFragment({
      id: apolloClient.cache.identify(user),
      fragment: gql`
        fragment UpdatedUser on User {
          id
          username
          email
          displayName
          role
          preferences
        }
      `,
      data: user,
    });
  },

  // Update workflow in cache
  updateWorkflow(workflow: any) {
    apolloClient.cache.writeFragment({
      id: apolloClient.cache.identify(workflow),
      fragment: gql`
        fragment UpdatedWorkflow on ResearchWorkflow {
          id
          name
          status
          progress
          updatedAt
        }
      `,
      data: workflow,
    });
  },

  // Add workflow execution to cache
  addWorkflowExecution(execution: any) {
    apolloClient.cache.modify({
      fields: {
        workflowExecutions(existingExecutions = []) {
          const newExecutionRef = apolloClient.cache.writeFragment({
            data: execution,
            fragment: gql`
              fragment NewExecution on WorkflowExecution {
                id
                workflowId
                status
                progress
                startedAt
              }
            `,
          });
          return [newExecutionRef, ...existingExecutions];
        },
      },
    });
  },
};

// Subscription helpers
export const subscriptionHelpers = {
  // Subscribe to workflow execution updates
  subscribeToWorkflowUpdates(workflowId: string, callback: (data: any) => void) {
    return graphqlHelpers.subscribe(
      gql`
        subscription WorkflowExecutionUpdates($workflowId: UUID!) {
          workflowExecutionUpdates(workflowId: $workflowId) {
            id
            workflowId
            status
            progress
            timestamp
            message
          }
        }
      `,
      { workflowId }
    ).subscribe({
      next: ({ data }) => callback(data.workflowExecutionUpdates),
      error: (error) => console.error('Subscription error:', error),
    });
  },

  // Subscribe to system metrics updates
  subscribeToSystemMetrics(callback: (data: any) => void) {
    return graphqlHelpers.subscribe(
      gql`
        subscription SystemMetricsUpdates {
          systemMetricsUpdates {
            timestamp
            metrics {
              cpuUsage
              memoryUsage
              activeWorkflows
              apiRequestsPerSecond
              errorRate
            }
            alertLevel
          }
        }
      `
    ).subscribe({
      next: ({ data }) => callback(data.systemMetricsUpdates),
      error: (error) => console.error('Metrics subscription error:', error),
    });
  },

  // Subscribe to API key usage updates
  subscribeToApiKeyUsage(keyId: string, callback: (data: any) => void) {
    return graphqlHelpers.subscribe(
      gql`
        subscription ApiKeyUsageUpdates($keyId: UUID!) {
          apiKeyUsageUpdates(keyId: $keyId) {
            keyId
            currentUsage
            usagePercentage
            timestamp
            rateLimitHit
          }
        }
      `,
      { keyId }
    ).subscribe({
      next: ({ data }) => callback(data.apiKeyUsageUpdates),
      error: (error) => console.error('API key usage subscription error:', error),
    });
  },
};

// Export default client
export default apolloClient;
