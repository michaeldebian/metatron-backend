---
description: Perform a comprehensive security review of code changes as a cybersecurity specialist
argument-hint: "PR-NUMBER"
---

# Security Review

Perform a comprehensive security review of code changes as a cybersecurity specialist.

## Usage

`/review-security {PR_NUMBER}`

## Instructions

### 1. Fetch PR Details from GitHub

```bash
gh pr view {PR_NUMBER} --json title,body,files,additions,deletions
gh pr diff {PR_NUMBER}
```

### 2. Analyze Code Changes
- Review all changed files from the diff
- Analyze code for security vulnerabilities
- Act as a cybersecurity specialist reviewing code for threats and compliance

### 3. Provide Review Results
- Provide a security score (X.X/10) and detailed findings
- Focus on identifying critical security issues

### 4. Post Review Comment to PR

```bash
gh pr comment {PR_NUMBER} --body "{REVIEW_CONTENT}"
```

## NexusFlow-Specific Security Checklist

### Proxy Route Security (`/api/proxy`, `/api/grpc-proxy`)
- [ ] SSRF protection: private IP ranges blocked (10.x, 172.16-31.x, 192.168.x, 127.x)
- [ ] URL validation: only http:// and https:// allowed
- [ ] Response size limits enforced (10MB cap)
- [ ] Timeout enforcement (30s)
- [ ] No credentials leaked in error responses

### Authentication & Secrets
- [ ] No credentials in logs, error messages, or console output
- [ ] SigV4 secret keys not exposed in client-side code
- [ ] GCP service account JSON validated before use
- [ ] Azure client secrets not leaked
- [ ] No sensitive data in `localStorage` without encryption

### Input Validation & Sanitization
- [ ] All user inputs validated and sanitized
- [ ] XSS prevention in response rendering
- [ ] Command injection prevention
- [ ] Path traversal protection
- [ ] No `eval()` usage (QuickJS WASM for sandboxed JS)

### Data Protection
- [ ] `nf-*` localStorage keys don't store unencrypted secrets
- [ ] No PII in browser console logs
- [ ] Secure error handling (no sensitive info in stack traces)

### Monaco Editor Security
- [ ] No arbitrary code execution from editor content
- [ ] Safe handling of pasted content
- [ ] Worker scripts loaded from trusted sources only

### Dependencies
- [ ] No new dependencies with known vulnerabilities
- [ ] Dependencies from trusted sources
- [ ] No unnecessary new dependencies added

## Review Output Format

Use this EXACT format for your review comment on the PR:

```
## Security Review

**Review Score: X.X/10**

**Critical Issues Found: X**

---

### Overall Assessment

[Brief 2-3 sentence summary of security posture]

**Security Posture:** [EXCELLENT/GOOD/ACCEPTABLE/NEEDS IMPROVEMENT/POOR]
**Vulnerability Risk:** [LOW/MEDIUM/HIGH/CRITICAL]

---

### Critical Issues

[If none, state "No critical issues found."]

#### 1. [Issue Title] (Severity: CRITICAL/HIGH/MEDIUM/LOW)
**Location:** `file/path.ext:line`
**Vulnerability Type:** [e.g., SSRF, XSS, Credential Exposure]
**Issue:** [Description]
**Impact:** [What happens if exploited]
**Recommendation:** [How to fix]

---

### Positive Observations

1. [Positive security aspect]
2. [Positive security aspect]

---

### Recommendations

**Immediate (Required):**
- [Must-fix security items]

**Short-term (Recommended):**
- [Should-fix security items]

**Long-term (Optional):**
- [Security enhancements]

---

### OWASP Compliance

**OWASP Top 10 2021:** [Compliance notes]

---

**Overall Recommendation:** [APPROVED/APPROVED WITH CHANGES/CHANGES REQUIRED/REJECTED]

**Reviewed by:** Security Specialist (Claude Code)
**Review Date:** [YYYY-MM-DD]
```
