use oxc_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::{self, Error},
};
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{context::LintContext, rule::Rule, AstNode};

#[derive(Debug, Error, Diagnostic)]
#[error("eslint-plugin-react(require-render-return):")]
#[diagnostic(severity(warning), help(""))]
struct RequireRenderReturnDiagnostic(#[label] pub Span);

#[derive(Debug, Default, Clone)]
pub struct RequireRenderReturn;

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
    RequireRenderReturn,
    correctness
);

impl Rule for RequireRenderReturn {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {}
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        r#"
			        class Hello extends React.Component {
			          render() {
			            return <div>Hello {this.props.name}</div>;
			          }
			        }
			      "#,
        r#"
			        class Hello extends React.Component {
			          render = () => {
			            return <div>Hello {this.props.name}</div>;
			          }
			        }
			      "#,
        r#"
			        class Hello extends React.Component {
			          render = () => (
			            <div>Hello {this.props.name}</div>
			          )
			        }
			      "#,
        r#"
			        var Hello = createReactClass({
			          displayName: 'Hello',
			          render: function() {
			            return <div></div>
			          }
			        });
			      "#,
        r#"
			        function Hello() {
			          return <div></div>;
			        }
			      "#,
        r#"
			        var Hello = () => (
			          <div></div>
			        );
			      "#,
        r#"
			        var Hello = createReactClass({
			          render: function() {
			            switch (this.props.name) {
			              case 'Foo':
			                return <div>Hello Foo</div>;
			              default:
			                return <div>Hello {this.props.name}</div>;
			            }
			          }
			        });
			      "#,
        r#"
			        var Hello = createReactClass({
			          render: function() {
			            if (this.props.name === 'Foo') {
			              return <div>Hello Foo</div>;
			            } else {
			              return <div>Hello {this.props.name}</div>;
			            }
			          }
			        });
			      "#,
        r#"
			        class Hello {
			          render() {}
			        }
			      "#,
        r#"class Hello extends React.Component {}"#,
        r#"var Hello = createReactClass({});"#,
        r#"
			        var render = require('./render');
			        var Hello = createReactClass({
			          render
			        });
			      "#,
        r#"
			        class Foo extends Component {
			          render
			        }
			      "#,
    ];

    let fail = vec![
        r#"
			        var Hello = createReactClass({
			          displayName: 'Hello',
			          render: function() {}
			        });
			      "#,
        r#"
			        class Hello extends React.Component {
			          render() {}
			        }
			      "#,
        r#"
			        class Hello extends React.Component {
			          render() {
			            const names = this.props.names.map(function(name) {
			              return <div>{name}</div>
			            });
			          }
			        }
			      "#,
        r#"
			        class Hello extends React.Component {
			          render = () => {
			            <div>Hello {this.props.name}</div>
			          }
			        }
			      "#,
    ];

    Tester::new_without_config(RequireRenderReturn::NAME, pass, fail).test_and_snapshot();
}
