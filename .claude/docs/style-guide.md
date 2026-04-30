## General principles

- Be concise, clear, pragmatic. Say what needs to be said, nothing more.
- No emojis.
- Avoid excessive markdown formatting. Use very few headers, sections, and horizontal rules; bold/italic even less.
- Do not repeat information that exists in other docs — link or reference instead.
- Do not create README files or documentation unless explicitly asked to.

## Code comments

Code should be self-documenting: Comments explain why, not what.

- Don't comment obvious code.
- Use comments when the reasoning behind a decision isn't apparent from the code itself. If in doubt, don't write.
- C# docs should use plain `///` comments, no XML tags.

GOOD:

```csharp
// GameLift enforces a 30 msg/sec global limit; throttle here to stay under.
await _rateLimiter.WaitAsync();
```

BAD:

```csharp
/// <summary>
/// Returns true if the type has at least one instance field.
/// </summary>
/// <param name="type">The type symbol to check.</param>
/// <returns>True if the type has instance fields.</returns>
private static bool HasInstanceFields(INamedTypeSymbol type)
```
