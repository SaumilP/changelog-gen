# Security Policy

## Reporting a Vulnerability

**Please do not open public issues for security vulnerabilities.**

Instead, please email security concerns to: security@example.com

Please include:

* Description of the vulnerability
* Steps to reproduce
* Potential impact
* Suggested fix (if any)

We will acknowledge receipt of your report within 48 hours and will send a more detailed response within 5 business days.

## Security Updates

Security updates will be released as patches (PATCH version bump) with a clear security advisory. We recommend:

* Always use the latest version
* Subscribe to release notifications
* Follow security best practices in your own code

## Known Issues

There are currently no known security issues.

## Version Support

| Version | Status | Support |
|---------|--------|---------|
| 1.x | Active | Security fixes + features |
| 0.x | Legacy | Security fixes only |

We recommend upgrading to the latest stable version as soon as possible.

## Dependencies

This project regularly audits dependencies for known vulnerabilities using `cargo audit`. The CI/CD pipeline runs security checks on every commit.
