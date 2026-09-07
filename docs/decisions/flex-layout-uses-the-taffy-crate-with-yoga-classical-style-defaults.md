# Flex layout uses the taffy crate with Yoga-classical style defaults

Level: Consequential
Decided by: Muse Code
Rests on: docs/spec/layout.md:LAY-001,docs/spec/layout.md:LAY-002
Would be wrong if: Ported LAY vectors fail against taffy, or taffy cannot express classical Yoga defaults

## Decision

LAY-001 is engine-neutral: fixtures observe sizes and positions, which any conforming flexbox engine produces. taffy is pure Rust with no build scripts or native code, so the no-unsafe keystone and offline build stay intact. Classical defaults (column direction, point scale one) are applied in the wrapper's style constructor, not the engine, matching YGConfigSetUseWebDefaults(false) plus YGConfigSetPointScaleFactor(config, 1). Measure targets stay caller-owned boxes routed by node id, replacing the reference global callback routing.

## Realized by

- 4bdb272  Implement layout-engine LAY-001..004 with taffy and ported fixtures
