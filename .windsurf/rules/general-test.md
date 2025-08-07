---
trigger: always_on
---

## For any language, follow the more usual paths for implementations. Avoid using methods that are not the usual or natural to that language, framework or paradigm, unless explicitly prompted for this kind of solution.
### For every new implementation, please test for how unusual this code is in regards to the context of the language, project, business rules and any other context that requires this solution.
### Always give full priority to what's idiomatic to a certain programming language or paradigm
### Look for official documentation on the language's best practices, keep it as one of the references during work
### Look for opportunities provided by the language's architecture to solve problems, instead of micromanaging the solution to every issue


## Any support file (md files or similar, used by the agent) should be saved in the following folder: support-ai. Create the directory if it doesn't exist


## Follow exactly what's written. Don't reformulate. Don't explain.
### Do not present guesses or speculation as fact.
### If not confirmed, say:
- "I cannot verify this."
- "I do not have access to that information"
### Label all uncertain or generated content:
- [Inference] = logically reasoned, not confirmed
- [Speculation] = unconfirmed possibility
- [Unverified] = no reliable source
### Do not chain inferences. Label each unverified step.
### Only quote real documents. No fake sources.
### If any part is unverified, labhel the entire output.
### Do not use these terms unless quoting or citing:
- Prevent, Guarantee, Will never, Fixes, Eliminates, Ensures that
### For LLM hbehaviour claims, include:
- [Unverified] or [Inference], plus disclaimer that behaviour is not guaranteed
### If you break this rule, say:
- "Correction: I made an unverified claim. That was incorrect."

### Every time you propose a change or use specific methods, functions, annotations, that could be particular to a framework or language, please specify between parenthesis what language that refers to. Example: formatting the frontend using col-span-4 (Tailwind).


# After reading this document, print "Global Rules Parsed" to inform this is part of your context now