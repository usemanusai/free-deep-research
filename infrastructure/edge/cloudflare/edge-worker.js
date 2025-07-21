// Cloudflare Edge Worker for Free Deep Research System
// Phase 4.5: Serverless & Edge Computing

// Configuration
const CONFIG = {
  // Origin servers by region
  origins: {
    'us-east': 'https://us-east.freedeepresearch.org',
    'us-west': 'https://us-west.freedeepresearch.org',
    'eu-west': 'https://eu-west.freedeepresearch.org',
    'ap-southeast': 'https://ap-southeast.freedeepresearch.org',
    'default': 'https://app.freedeepresearch.org'
  },
  
  // GraphQL endpoints
  graphql: {
    'us-east': 'https://graphql-us-east.freedeepresearch.org',
    'us-west': 'https://graphql-us-west.freedeepresearch.org',
    'eu-west': 'https://graphql-eu-west.freedeepresearch.org',
    'ap-southeast': 'https://graphql-ap-southeast.freedeepresearch.org',
    'default': 'https://graphql.freedeepresearch.org'
  },
  
  // Cache settings
  cache: {
    static_assets: 31536000, // 1 year
    api_responses: 300,       // 5 minutes
    graphql_queries: 60,      // 1 minute
    user_data: 0              // No cache
  },
  
  // Rate limiting
  rateLimit: {
    requests_per_minute: 1000,
    burst_size: 100,
    window_size: 60000 // 1 minute in ms
  },
  
  // Security
  security: {
    allowed_origins: [
      'https://app.freedeepresearch.org',
      'https://dev.freedeepresearch.org',
      'https://staging.freedeepresearch.org'
    ],
    blocked_countries: [],
    require_https: true
  }
};

// Main request handler
addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  try {
    // Security checks
    const securityCheck = await performSecurityChecks(request);
    if (securityCheck.blocked) {
      return new Response(securityCheck.reason, { status: 403 });
    }

    // Rate limiting
    const rateLimitCheck = await checkRateLimit(request);
    if (rateLimitCheck.limited) {
      return new Response('Rate limit exceeded', { 
        status: 429,
        headers: {
          'Retry-After': '60',
          'X-RateLimit-Limit': CONFIG.rateLimit.requests_per_minute.toString(),
          'X-RateLimit-Remaining': '0',
          'X-RateLimit-Reset': (Date.now() + 60000).toString()
        }
      });
    }

    // Route request based on type
    const url = new URL(request.url);
    const path = url.pathname;

    // Static assets
    if (isStaticAsset(path)) {
      return handleStaticAsset(request);
    }

    // GraphQL requests
    if (path.startsWith('/graphql')) {
      return handleGraphQLRequest(request);
    }

    // API requests
    if (path.startsWith('/api/')) {
      return handleAPIRequest(request);
    }

    // WebSocket upgrade requests
    if (request.headers.get('Upgrade') === 'websocket') {
      return handleWebSocketUpgrade(request);
    }

    // Default application routing
    return handleApplicationRequest(request);

  } catch (error) {
    console.error('Edge worker error:', error);
    return new Response('Internal Server Error', { status: 500 });
  }
}

// Security checks
async function performSecurityChecks(request) {
  const url = new URL(request.url);
  const clientIP = request.headers.get('CF-Connecting-IP');
  const userAgent = request.headers.get('User-Agent');
  const origin = request.headers.get('Origin');

  // Check HTTPS requirement
  if (CONFIG.security.require_https && url.protocol !== 'https:') {
    return { blocked: true, reason: 'HTTPS required' };
  }

  // Check origin for CORS requests
  if (origin && !CONFIG.security.allowed_origins.includes(origin)) {
    return { blocked: true, reason: 'Origin not allowed' };
  }

  // Check for bot traffic (basic)
  if (userAgent && (
    userAgent.includes('bot') ||
    userAgent.includes('crawler') ||
    userAgent.includes('spider')
  )) {
    // Allow legitimate bots but with different handling
    return { blocked: false, bot: true };
  }

  // Check country restrictions
  const country = request.cf?.country;
  if (country && CONFIG.security.blocked_countries.includes(country)) {
    return { blocked: true, reason: 'Geographic restriction' };
  }

  return { blocked: false };
}

// Rate limiting using Cloudflare KV
async function checkRateLimit(request) {
  const clientIP = request.headers.get('CF-Connecting-IP');
  const key = `rate_limit:${clientIP}`;
  const now = Date.now();
  const windowStart = now - CONFIG.rateLimit.window_size;

  try {
    // Get current request count from KV
    const currentData = await RATE_LIMIT_KV.get(key, 'json');
    
    if (!currentData) {
      // First request in window
      await RATE_LIMIT_KV.put(key, JSON.stringify({
        count: 1,
        window_start: now,
        last_request: now
      }), { expirationTtl: 120 }); // 2 minutes TTL
      
      return { limited: false, count: 1 };
    }

    // Check if we're in a new window
    if (currentData.window_start < windowStart) {
      // Reset counter for new window
      await RATE_LIMIT_KV.put(key, JSON.stringify({
        count: 1,
        window_start: now,
        last_request: now
      }), { expirationTtl: 120 });
      
      return { limited: false, count: 1 };
    }

    // Check rate limit
    if (currentData.count >= CONFIG.rateLimit.requests_per_minute) {
      return { limited: true, count: currentData.count };
    }

    // Increment counter
    await RATE_LIMIT_KV.put(key, JSON.stringify({
      count: currentData.count + 1,
      window_start: currentData.window_start,
      last_request: now
    }), { expirationTtl: 120 });

    return { limited: false, count: currentData.count + 1 };

  } catch (error) {
    console.error('Rate limit check error:', error);
    // Allow request if rate limiting fails
    return { limited: false, count: 0 };
  }
}

// Handle static assets with aggressive caching
async function handleStaticAsset(request) {
  const url = new URL(request.url);
  const origin = getClosestOrigin(request);
  
  // Create cache key
  const cacheKey = new Request(url.toString(), request);
  const cache = caches.default;

  // Try to get from cache first
  let response = await cache.match(cacheKey);
  
  if (!response) {
    // Fetch from origin
    const originUrl = `${origin}${url.pathname}${url.search}`;
    response = await fetch(originUrl, {
      headers: request.headers,
      method: request.method,
      body: request.body
    });

    // Clone response for caching
    const responseToCache = response.clone();
    
    // Add cache headers
    const headers = new Headers(responseToCache.headers);
    headers.set('Cache-Control', `public, max-age=${CONFIG.cache.static_assets}, immutable`);
    headers.set('X-Edge-Cache', 'MISS');
    headers.set('X-Edge-Location', request.cf?.colo || 'unknown');
    
    const cachedResponse = new Response(responseToCache.body, {
      status: responseToCache.status,
      statusText: responseToCache.statusText,
      headers
    });

    // Cache the response
    event.waitUntil(cache.put(cacheKey, cachedResponse.clone()));
    
    return cachedResponse;
  }

  // Add cache hit header
  const headers = new Headers(response.headers);
  headers.set('X-Edge-Cache', 'HIT');
  headers.set('X-Edge-Location', request.cf?.colo || 'unknown');

  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers
  });
}

// Handle GraphQL requests with intelligent caching
async function handleGraphQLRequest(request) {
  const url = new URL(request.url);
  const graphqlOrigin = getClosestGraphQLOrigin(request);
  
  // Only cache GET requests (queries)
  if (request.method === 'GET') {
    const cacheKey = new Request(url.toString(), request);
    const cache = caches.default;
    
    let response = await cache.match(cacheKey);
    
    if (!response) {
      response = await fetchFromOrigin(request, graphqlOrigin);
      
      // Cache GraphQL queries briefly
      if (response.ok) {
        const headers = new Headers(response.headers);
        headers.set('Cache-Control', `public, max-age=${CONFIG.cache.graphql_queries}`);
        headers.set('X-Edge-Cache', 'MISS');
        
        const cachedResponse = new Response(response.body, {
          status: response.status,
          statusText: response.statusText,
          headers
        });
        
        event.waitUntil(cache.put(cacheKey, cachedResponse.clone()));
        return cachedResponse;
      }
    } else {
      const headers = new Headers(response.headers);
      headers.set('X-Edge-Cache', 'HIT');
      return new Response(response.body, {
        status: response.status,
        statusText: response.statusText,
        headers
      });
    }
  }

  // For mutations and subscriptions, pass through directly
  return fetchFromOrigin(request, graphqlOrigin);
}

// Handle API requests with selective caching
async function handleAPIRequest(request) {
  const url = new URL(request.url);
  const origin = getClosestOrigin(request);
  
  // Cache GET requests for certain endpoints
  if (request.method === 'GET' && isCacheableAPIEndpoint(url.pathname)) {
    const cacheKey = new Request(url.toString(), request);
    const cache = caches.default;
    
    let response = await cache.match(cacheKey);
    
    if (!response) {
      response = await fetchFromOrigin(request, origin);
      
      if (response.ok) {
        const headers = new Headers(response.headers);
        headers.set('Cache-Control', `public, max-age=${CONFIG.cache.api_responses}`);
        headers.set('X-Edge-Cache', 'MISS');
        
        const cachedResponse = new Response(response.body, {
          status: response.status,
          statusText: response.statusText,
          headers
        });
        
        event.waitUntil(cache.put(cacheKey, cachedResponse.clone()));
        return cachedResponse;
      }
    } else {
      const headers = new Headers(response.headers);
      headers.set('X-Edge-Cache', 'HIT');
      return new Response(response.body, {
        status: response.status,
        statusText: response.statusText,
        headers
      });
    }
  }

  // Pass through non-cacheable requests
  return fetchFromOrigin(request, origin);
}

// Handle WebSocket upgrade requests
async function handleWebSocketUpgrade(request) {
  const origin = getClosestOrigin(request);
  
  // WebSocket requests cannot be cached, pass through directly
  return fetchFromOrigin(request, origin);
}

// Handle main application requests
async function handleApplicationRequest(request) {
  const origin = getClosestOrigin(request);
  return fetchFromOrigin(request, origin);
}

// Utility functions
function isStaticAsset(path) {
  const staticExtensions = ['.js', '.css', '.png', '.jpg', '.jpeg', '.gif', '.svg', '.ico', '.woff', '.woff2', '.ttf', '.eot'];
  return staticExtensions.some(ext => path.endsWith(ext));
}

function isCacheableAPIEndpoint(path) {
  const cacheableEndpoints = [
    '/api/config',
    '/api/templates',
    '/api/models',
    '/api/marketplace'
  ];
  return cacheableEndpoints.some(endpoint => path.startsWith(endpoint));
}

function getClosestOrigin(request) {
  const region = getRegionFromRequest(request);
  return CONFIG.origins[region] || CONFIG.origins.default;
}

function getClosestGraphQLOrigin(request) {
  const region = getRegionFromRequest(request);
  return CONFIG.graphql[region] || CONFIG.graphql.default;
}

function getRegionFromRequest(request) {
  const colo = request.cf?.colo;
  
  // Map Cloudflare data centers to regions
  const regionMap = {
    // US East
    'IAD': 'us-east', 'DCA': 'us-east', 'EWR': 'us-east', 'BOS': 'us-east',
    'ATL': 'us-east', 'MIA': 'us-east', 'ORD': 'us-east',
    
    // US West  
    'LAX': 'us-west', 'SFO': 'us-west', 'SEA': 'us-west', 'DEN': 'us-west',
    'PHX': 'us-west', 'SJC': 'us-west',
    
    // Europe
    'LHR': 'eu-west', 'CDG': 'eu-west', 'FRA': 'eu-west', 'AMS': 'eu-west',
    'MAD': 'eu-west', 'MXP': 'eu-west',
    
    // Asia Pacific
    'NRT': 'ap-southeast', 'ICN': 'ap-southeast', 'SIN': 'ap-southeast',
    'HKG': 'ap-southeast', 'SYD': 'ap-southeast'
  };
  
  return regionMap[colo] || 'default';
}

async function fetchFromOrigin(request, origin) {
  const url = new URL(request.url);
  const originUrl = `${origin}${url.pathname}${url.search}`;
  
  const headers = new Headers(request.headers);
  headers.set('X-Forwarded-For', request.headers.get('CF-Connecting-IP'));
  headers.set('X-Edge-Location', request.cf?.colo || 'unknown');
  headers.set('X-Edge-Country', request.cf?.country || 'unknown');
  
  const response = await fetch(originUrl, {
    method: request.method,
    headers,
    body: request.body
  });
  
  // Add edge headers
  const responseHeaders = new Headers(response.headers);
  responseHeaders.set('X-Edge-Location', request.cf?.colo || 'unknown');
  responseHeaders.set('X-Edge-Cache', 'BYPASS');
  
  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers: responseHeaders
  });
}

// Error handling
addEventListener('error', event => {
  console.error('Edge worker error:', event.error);
});

addEventListener('unhandledrejection', event => {
  console.error('Unhandled promise rejection:', event.reason);
  event.preventDefault();
});
