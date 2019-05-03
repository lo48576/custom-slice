# Change Log

## [Unreleased]

* Some new derive targets are added.

### Added
* Some new derive targets are added.
    + `As{Ref,Mut}Slice{,Inner}`
    + `{Try,}FromInner`
    + `IntoInner`
    + `Partial{Eq,Ord}{,Bulk,InnerBulk}`
    + `Deref{,Mut}`
        + Previously they are supported only for owned types.
          Now they are also supported for slice types.

## [0.1.0]

First release.

[Unreleased]: <https://github.com/lo48576/custom-slice/compare/v0.1.0...develop>
[0.1.0]: <https://github.com/lo48576/custom-slice/releases/tag/v0.1.0>
