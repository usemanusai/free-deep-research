# PRD Generate Task

## Purpose

- Transform inputs into core product definition documents conforming to a PRD template
- Define clear MVP scope focused on essential functionality
- Provide foundation for Architect and Design Architect to help create technical artifacts which will in turn later draft further details for very junior engineers or simple dev ai agents.

## Instructions

### 1. Review Inputs

Review all provided inputs including project brief, research documents, prd template and user ideas to guide PRD generation.

### 2. Determine Interaction Mode

Confirm with the user their preferred interaction style:

- **Incremental:** Work through sections one at a time via chat messages as defined in the template.

- **YOLO Mode:** Draft the complete PRD making assumptions as necessary. Present full document at once, noting which sections required assumptions.

### 3. Execute Template

- Use the `prd-tmpl` template (or user-specified alternative template)
- Follow all embedded LLM instructions within the template
- Template contains section-specific guidance and examples

### 4. Template Processing Notes

- **Incremental Mode**: Present each section for review before proceeding
- **YOLO Mode**: Generate all sections, then review with user

Process all template elements according to `templates#template-format` conventions.

**CRITICAL: Never display or output template markup formatting, LLM instructions or examples - they MUST be used by you the agent only, AND NEVER shown to users in chat or document output**

**Content Presentation Guidelines:**

- Present only the final, clean content to users
- Replace template variables with actual project-specific content
- Process all conditional logic internally - show only relevant sections
- For Canvas mode: Update the document with clean, formatted content only

### 7. Prepare Handoffs

Based on PRD content, prepare appropriate next-step prompts:

**If UI Component Exists:**

1. Add Design Architect prompt in designated template section
2. Recommend: User engages Design Architect first for UI/UX Specification
3. Then proceed to Architect with enriched PRD

**If No UI Component:**

- Add Architect prompt in designated template section
- Recommend proceeding directly to Architect

### 8. Validate with Checklist

- Run the `pm-checklist` against completed PRD
- Document completion status for each checklist item
- Present summary by section, address any deficiencies
- Generate final checklist report with findings and resolutions

### 9. Final Presentation

**General Guidelines:**

- Present complete documents in clean, full format
- DO NOT truncate unchanged information
- Begin directly with content (no introductory text needed)
- Ensure all template sections are properly filled
- **NEVER show template markup, instructions, or processing directives to users**

## Key Resources

- **Default Template:** `templates#prd-tmpl`
- **Validation:** `checklists#pm-checklist`
- **User Preferences:** `data#technical-preferences`
- **Elicitation Protocol:** `tasks#advanced-elicitation`

## Important Notes

- This task is template-agnostic - users may specify custom templates
- All detailed instructions are embedded in templates, not this task file
- Focus on orchestration and workflow
- **Template markup is for AI processing only - users should never see output indicators from templates#template-format**
