# MD Template Format:

- {{placeholder}} = Simple text replacement placeholder
- [[LLM: instruction]] = Instructions for the LLM (not included in output)
- <<REPEAT: section_name>> ... <</REPEAT>> = Repeating section
- ^^CONDITION: condition_name^^ ... ^^/CONDITION: condition_name^^ = Conditional section that will render if the condition_name logically applies
- @{example: content} = Single line example content for LLM guidance - do not render
- @{example} ... @{/example} = Multi-line example content for LLM guidance - do not render

## Critical Template Usage Rules

- CRITICAL: Never display or output template markup formatting, LLM instructions or examples
  - they MUST be used by you the agent only, AND NEVER shown to users in chat or documented output\*\*
- Present only the final, clean content to users
- Replace template variables with actual project-specific content
- Show examples only when they add value, without the markup
- Process all conditional logic internally - show only relevant sections
- For Canvas mode: Update the document with clean, formatted content only

@{example}

# My Template Foo

[[LLM: Check the current system date and if the user name is unknown, just say hello]]
Hello {{users name}}, this is your foo report for {{todays date}}

<<REPEAT: single_foo>>
[[LLM: For Each Foo, Create a matching creative Bar]]

## Foo: {{Bar}}

<</REPEAT>>

^^CONDITION: if_BAZ_exists^^

## BAZ

### You haz BAZ! Here is your daily Baz Forecast!

[[LLM: Give the user their daily baz report here]]
^^/CONDITION: if_BAZ_exists^^

@{/example}
