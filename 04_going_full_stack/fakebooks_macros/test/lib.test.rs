use fakebooks_macros::*;
use leptos::{Scope, RuntimeId, ScopeId, view};

#[test]
fn test_macro_describe() {
    // Arrange
    let cx = Scope { runtime: RuntimeId::default(), id: ScopeId::default() };

    // Act
    let actual = kaas! {
        "../../",
        <p>"Some text in the paragraph"</p>
    };

    // Assert
    // assert_eq!(actual, 10);
}