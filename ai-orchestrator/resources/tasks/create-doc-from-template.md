# Create Document from Template Task

## Purpose

- Generate documents from any specified template following embedded instructions
- Support multiple document types through template-driven approach
- Enable any persona to create consistent, well-structured documents

## Instructions

### 1. Identify Template and Context

- Determine which template to use (user-provided or list available for selection to user)

  - agent-config specific agents will list what docs they have available under this task, for each item consider it a unique task. So if the user had for example:

    @{example}

  - tasks:

  - [Create Document](tasks#create-doc-from-template):

    - [Prd](templates#prd-tmpl)

    - [Architecture](templates#architecture-tmpl)

    @{/example}

    you would list `Create Document PRD` and `Create Document Architecture` as tasks the agent could perform.

- Gather all relevant inputs, or ask for them, or else rely on user providing necessary details to complete the document
- Understand the document purpose and target audience

### 2. Determine Interaction Mode

Confirm with the user their preferred interaction style:

- **Incremental:** Work through chunks of the document.
- **YOLO Mode:** Draft complete document making reasonable assumptions in one shot. (Can be entered also after starting incremental by just typing /yolo)

### 3. Execute Template

- Load specified template from `templates#*` or the /templates directory
- Follow ALL embedded LLM instructions within the template
- Process template markup according to `templates#template-format` conventions

### 4. Template Processing Rules

**CRITICAL: Never display template markup, LLM instructions, or examples to users**

- Replace all {{placeholders}} with actual content
- Execute all [[LLM: instructions]] internally
- Process <<REPEAT>> sections as needed
- Evaluate ^^CONDITION^^ blocks and include only if applicable
- Use @{examples} for guidance but never output them

### 5. Content Generation

- **Incremental Mode**: Present each major section for review before proceeding
- **YOLO Mode**: Generate all sections, then review complete document with user
- Apply any elicitation protocols specified in template
- Incorporate user feedback and iterate as needed

### 6. Validation

If template specifies a checklist:

- Run the appropriate checklist against completed document
- Document completion status for each item
- Address any deficiencies found
- Present validation summary to user

### 7. Final Presentation

- Present clean, formatted content only
- Ensure all sections are complete
- DO NOT truncate or summarize content
- Begin directly with document content (no preamble)
- Include any handoff prompts specified in template

## Key Resources

- **Template Format:** `templates#template-format`
- **Available Templates:** All files in `templates#` directory
- **Checklists:** As specified by template or persona
- **User Preferences:** `data#technical-preferences`

## Important Notes

- This task is template and persona agnostic
- All specific instructions are embedded in templates
- Focus on faithful template execution and clean output
- Template markup is for AI processing only - never expose to users
