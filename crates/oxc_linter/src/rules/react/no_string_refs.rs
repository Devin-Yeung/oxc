use oxc_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::{self, Error},
};
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{context::LintContext, rule::Rule, AstNode};

#[derive(Debug, Error, Diagnostic)]
#[error("")]
#[diagnostic(severity(warning), help(""))]
struct NoStringRefsDiagnostic(#[label] pub Span);

#[derive(Debug, Default, Clone)]
pub struct NoStringRefs;

declare_oxc_lint!(
    /// ### What it does
    ///
    ///
    /// ### Why is this bad?
    ///
    ///
    /// ### Example
    /// ```javascript
    /// ```
    NoStringRefs,
    correctness
);

impl Rule for NoStringRefs {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {}
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        (
            "
			        var Hello = createReactClass({
			          componentDidMount: function() {
			            var component = this.hello;
			          },
			          render: function() {
			            return <div ref={c => this.hello = c}>Hello {this.props.name}</div>;
			          }
			        });
			      ",
            None,
        ),
        (
            "
			        var Hello = createReactClass({
			          render: function() {
			            return <div ref={`hello`}>Hello {this.props.name}</div>;
			          }
			        });
			      ",
            None,
        ),
        (
            "
			        var Hello = createReactClass({
			          render: function() {
			            return <div ref={`hello${index}`}>Hello {this.props.name}</div>;
			          }
			        });
			      ",
            None,
        ),
    ];

    let fail = vec![
        (
            "
			        var Hello = createReactClass({
			          componentDidMount: function() {
			            var component = this.refs.hello;
			          },
			          render: function() {
			            return <div>Hello {this.props.name}</div>;
			          }
			        });
			      ",
            None,
        ),
        (
            "
			        var Hello = createReactClass({
			          render: function() {
			            return <div ref=\"hello\">Hello {this.props.name}</div>;
			          }
			        });
			      ",
            None,
        ),
        (
            "
			        var Hello = createReactClass({
			          render: function() {
			            return <div ref={'hello'}>Hello {this.props.name}</div>;
			          }
			        });
			      ",
            None,
        ),
        (
            "
			        var Hello = createReactClass({
			          componentDidMount: function() {
			            var component = this.refs.hello;
			          },
			          render: function() {
			            return <div ref=\"hello\">Hello {this.props.name}</div>;
			          }
			        });
			      ",
            None,
        ),
        (
            "
			        var Hello = createReactClass({
			          componentDidMount: function() {
			          var component = this.refs.hello;
			          },
			          render: function() {
			            return <div ref={`hello`}>Hello {this.props.name}</div>;
			          }
			        });
			      ",
            Some(serde_json::json!([{ "noTemplateLiterals": true }])),
        ),
        (
            "
			        var Hello = createReactClass({
			          componentDidMount: function() {
			          var component = this.refs.hello;
			          },
			          render: function() {
			            return <div ref={`hello${index}`}>Hello {this.props.name}</div>;
			          }
			        });
			      ",
            Some(serde_json::json!([{ "noTemplateLiterals": true }])),
        ),
    ];

    Tester::new(NoStringRefs::NAME, pass, fail).test_and_snapshot();
}
