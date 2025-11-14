# CLAUDE.md - AI Assistant Guide for mod_clod

> **Last Updated**: 2025-11-14
> **Repository**: mod_clod
> **Status**: New/Initial Setup

## Overview

This document provides comprehensive guidance for AI assistants working on the mod_clod repository. It covers codebase structure, development workflows, conventions, and important context.

---

## Project Information

### Project Status
- **Current State**: New repository, initial setup phase
- **Primary Language**: TBD (to be determined based on project needs)
- **Project Type**: TBD

### Purpose
*This section should be updated once the project purpose is defined.*

### Key Technologies
*To be filled in as the tech stack is determined:*
- Primary language:
- Framework(s):
- Database:
- Build tools:
- Testing framework:
- Deployment platform:

---

## Repository Structure

```
mod_clod/
├── .git/                 # Git version control
├── CLAUDE.md            # This file - AI assistant guide
└── [To be populated]
```

### Expected Directory Structure
*This should be updated as the project structure develops:*

```
mod_clod/
├── src/                 # Source code
├── tests/               # Test files
├── docs/                # Documentation
├── config/              # Configuration files
├── scripts/             # Build and utility scripts
├── .github/             # GitHub workflows and templates
├── README.md            # Project readme
├── CLAUDE.md            # This file
└── [other directories]
```

---

## Development Workflow

### Branch Strategy

**Current Branch**: `claude/claude-md-mhzhgdntqe0uafpq-01Rur6ooUN3QZFYr4ZpqDL8c`

#### Branch Naming Conventions
- `main` or `master` - Production-ready code
- `develop` - Integration branch for features
- `feature/[name]` - New feature development
- `bugfix/[name]` - Bug fixes
- `hotfix/[name]` - Urgent production fixes
- `claude/[session-id]` - AI assistant working branches

#### Git Workflow
1. **Always work on designated feature/claude branches**
2. **Never commit directly to main/master**
3. **Commit frequently with clear, descriptive messages**
4. **Push to remote when work is complete**

### Commit Message Format

Use conventional commit format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements

**Examples:**
```
feat(auth): add user authentication module

Implements JWT-based authentication with refresh tokens.
Includes login, logout, and token refresh endpoints.

Closes #123
```

```
fix(api): resolve race condition in data fetch

Adds proper error handling and retry logic to prevent
concurrent request conflicts.
```

### Pull Request Process

1. **Before Creating PR:**
   - Ensure all tests pass
   - Run linters and formatters
   - Update documentation if needed
   - Review your own changes first

2. **PR Title and Description:**
   - Use clear, descriptive titles
   - Include summary of changes
   - Reference related issues
   - Add test plan/checklist

3. **PR Checklist:**
   - [ ] Code follows project conventions
   - [ ] Tests added/updated
   - [ ] Documentation updated
   - [ ] No breaking changes (or documented)
   - [ ] All CI checks pass

---

## Code Conventions

### General Principles
- **Clarity over cleverness**: Write code that is easy to understand
- **DRY (Don't Repeat Yourself)**: Extract common logic
- **SOLID principles**: Follow object-oriented design principles
- **Error handling**: Always handle errors gracefully
- **Security first**: Validate inputs, sanitize outputs

### Code Style
*To be defined based on chosen language/framework:*
- Indentation: (spaces/tabs, size)
- Line length: (max characters)
- Naming conventions:
  - Variables:
  - Functions:
  - Classes:
  - Constants:
  - Files:

### Documentation Standards
- **Every public function/method should have documentation**
- **Complex logic should include inline comments**
- **README files for each major module/package**

### Testing Requirements
- **Unit tests**: For all business logic
- **Integration tests**: For API endpoints and database interactions
- **Test coverage**: Aim for >80% coverage
- **Test naming**: Use descriptive names that explain what is being tested

---

## File Organization

### Configuration Files
*Place all configuration in version control (except secrets):*
- Use environment variables for secrets
- Maintain example config files (e.g., `.env.example`)
- Document all configuration options

### Dependency Management
- Keep dependencies up to date
- Document why each dependency is needed
- Prefer well-maintained, popular libraries
- Lock dependency versions for reproducibility

---

## Development Environment

### Prerequisites
*To be filled in:*
- Runtime version:
- Required tools:
- System dependencies:

### Setup Instructions
```bash
# Clone repository
git clone [repository-url]
cd mod_clod

# Install dependencies
# [To be filled in based on tech stack]

# Setup environment
# [Copy and configure environment files]

# Run tests
# [Test command]

# Start development server
# [Start command]
```

---

## Testing Strategy

### Test Organization
```
tests/
├── unit/              # Unit tests
├── integration/       # Integration tests
├── e2e/              # End-to-end tests
├── fixtures/         # Test data and fixtures
└── helpers/          # Test utilities
```

### Running Tests
```bash
# Run all tests
# [command]

# Run specific test suite
# [command]

# Run with coverage
# [command]

# Watch mode for development
# [command]
```

### Test Data
- Use fixtures for consistent test data
- Mock external services
- Clean up test data after each test

---

## Security Considerations

### Important Security Practices
1. **Never commit secrets or credentials**
2. **Validate all user inputs**
3. **Sanitize outputs to prevent XSS**
4. **Use parameterized queries to prevent SQL injection**
5. **Implement proper authentication and authorization**
6. **Keep dependencies updated for security patches**
7. **Follow OWASP Top 10 guidelines**

### Common Vulnerabilities to Avoid
- Command injection
- SQL injection
- XSS (Cross-Site Scripting)
- CSRF (Cross-Site Request Forgery)
- Path traversal
- Insecure deserialization
- Insufficient logging and monitoring

---

## Performance Considerations

- **Database queries**: Use indexes, avoid N+1 queries
- **Caching**: Implement where appropriate
- **Async operations**: Use for I/O-bound tasks
- **Resource management**: Clean up connections, file handles, etc.
- **Monitoring**: Implement performance monitoring

---

## API Design (if applicable)

### RESTful Conventions
- Use appropriate HTTP methods (GET, POST, PUT, PATCH, DELETE)
- Use meaningful resource names
- Version your API (`/api/v1/...`)
- Return appropriate status codes
- Include pagination for list endpoints
- Implement rate limiting

### Response Format
```json
{
  "success": true,
  "data": {},
  "error": null,
  "meta": {
    "timestamp": "2025-11-14T00:00:00Z",
    "version": "1.0"
  }
}
```

---

## Database Conventions (if applicable)

### Schema Design
- Use meaningful table and column names
- Implement proper indexes
- Use foreign keys for referential integrity
- Document schema changes

### Migrations
- Always use migrations for schema changes
- Never modify existing migrations
- Include both up and down migrations
- Test migrations before committing

---

## Deployment Process

### Build Process
```bash
# Build command
# [To be filled in]
```

### Deployment Checklist
- [ ] All tests passing
- [ ] Code reviewed and approved
- [ ] Documentation updated
- [ ] Environment variables configured
- [ ] Database migrations applied
- [ ] Monitoring and logging configured
- [ ] Rollback plan prepared

---

## Debugging and Troubleshooting

### Common Issues and Solutions
*To be populated as issues are encountered*

### Logging
- Use appropriate log levels (DEBUG, INFO, WARN, ERROR)
- Include context in log messages
- Never log sensitive information
- Use structured logging where possible

### Debug Tools
*List relevant debugging tools and commands*

---

## AI Assistant Specific Guidelines

### Code Review Focus Areas
1. **Security**: Check for common vulnerabilities
2. **Performance**: Identify potential bottlenecks
3. **Maintainability**: Ensure code is readable and well-documented
4. **Testing**: Verify adequate test coverage
5. **Error Handling**: Ensure all error cases are handled

### Before Making Changes
1. **Understand the context**: Read related code
2. **Check existing patterns**: Follow established conventions
3. **Consider impact**: Think about breaking changes
4. **Test thoroughly**: Ensure changes don't break existing functionality

### When Stuck or Uncertain
1. **Search the codebase**: Look for similar patterns
2. **Check documentation**: Review README, CLAUDE.md, comments
3. **Ask for clarification**: Don't make assumptions
4. **Start small**: Make incremental changes

### Task Management
- Use TodoWrite tool for complex, multi-step tasks
- Break down large tasks into smaller, manageable steps
- Mark tasks as completed immediately after finishing
- Keep only one task in progress at a time

---

## Resources and References

### Documentation
- Project README: [To be created]
- API Documentation: [To be created]
- Architecture Docs: [To be created]

### External Resources
- [Link to relevant technology docs]
- [Link to framework guides]
- [Link to best practices]

---

## Change Log

### 2025-11-14
- Initial CLAUDE.md creation
- Set up template structure
- Defined basic conventions and workflows

---

## Notes for Future Updates

This document should be updated:
- When new technologies are added to the stack
- When architectural decisions are made
- When new conventions are established
- When common issues/solutions are discovered
- At major project milestones

**Keep this document current and accurate to maximize its value for AI assistants!**
