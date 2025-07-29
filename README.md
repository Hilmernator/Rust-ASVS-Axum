# 🔐 Rusty API – Path Traversal Secure-by-Design Demo

This is an intentionally vulnerable-to-secure Rust-based API demo project, designed to **illustrate how to build secure file-based APIs by design** using selected controls from the [OWASP Application Security Verification Standard (ASVS)](https://owasp.org/www-project-application-security-verification-standard/).

---

## 🎯 Project Purpose

The goal is to **demonstrate API-level security controls** with a focus on:

- ✅ Preventing **path traversal** vulnerabilities
- ✅ Implementing **secure file access** patterns
- ✅ Applying selected OWASP **ASVS Level 3** requirements

This project is ideal for:
- Security education and awareness
- Showcasing secure coding patterns
- Live demonstrations in YouTube content
- Practicing path validation and canonicalization in Rust

---

## 🚧 Security Scope Disclaimer

This project is **not intended for production** use.

The following areas are intentionally out of scope to focus narrowly on path traversal and API security:

| ❌ Not Implemented                        | Reason                                                    |
|------------------------------------------|------------------------------------------------------------|
| Authentication (e.g., OAuth, API keys)   | Out of scope – focus is on API logic                      |
| Authorization (e.g., role-based access)  | Not relevant for this demo                                |
| Transport security (HTTPS/TLS)           | Handled by reverse proxies or infra in production         |
| Rate limiting / throttling               | Not implemented                                           |
| Logging, monitoring, or audit trails     | Out of scope                                              |
| Complete input validation                | Only alias/path are sanitized                             |

---

## ✅ Implemented ASVS Requirements

| ASVS ID    | Description                                                                 | Status  |
|------------|-----------------------------------------------------------------------------|---------|
| **5.1.1**  | Validate input using whitelists                                              | ✅ Regex used on   `alias` and `path`     |

| **5.1.4**  | Avoid using untrusted input for file access                                  | ✅ All input sanitized before use       |

| **5.3.3**  | Canonicalize paths and enforce base path confinement                         | ✅ Implemented in `secure_join_path()` |

| **12.1.1** | Use `Content-Disposition: attachment` for file downloads                    | ✅ Used in `/export` route              |

| **12.4.1** | Avoid direct file access via predictable URLs                               | ✅ Only accessible via safe endpoints   |

| **12.5.1** | Mitigate path traversal attempts (e.g., `../`)                              | ✅ Regex + blacklist + canonical check  |


---

## 👨‍💻 Author

**Rasmus Hilmersson**  
Application Security Architect & Rust Developer  
📹 Creating educational security content on YouTube (https://www.youtube.com/@0Trustwastaken)

---

