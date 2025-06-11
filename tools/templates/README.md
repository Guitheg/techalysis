Possible inputs
---
- indicator_name
- IndicatorName
- INDICATORNAME
- ContributorName (default: 'your git name' or 'Unknown')

Usage
---

Uses the `Possible inputs` in the template files inside `${}` such as:

```
/// ${INDICATORNAME} function
fn ${indicator_name}(
    //TODO
) -> ${IndicatorName}State {
    ...
}
```

The previous sample will produce with "ema" as `indicator name`:
```
/// EMA function
fn ema(
    //TODO
) -> EmaState {
    ...
}
```