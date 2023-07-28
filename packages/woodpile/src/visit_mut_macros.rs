#[macro_export]
/// Macro wraps each visit_mut_NODE
macro_rules! write_visit_mut {
    // if the node name contains capital letter should not be snakecased, i.e. JSXText
    ($capital: ident, $ty: ident) => {
        paste::paste! {
            #[typed_visit(Visitor)]
            fn [<visit_mut_$capital:lower _$ty:snake>](&mut self, n: &mut [<$capital:upper $ty>]) {
              crate::visit_mut_macros_helper::call_visit_mut_inner(
                    &self.visitor,
                    &self.visitor_context,
                    n,
                    &format!("visit{}", stringify!([<$capital$ty:camel>])),
                    stringify!([<$capital:upper $ty>]),
                    None
                );

                n.visit_mut_children_with(self);
            }
        }
    };
    // Noraml case, i.e. Expr
    ($ty:ident) => {
        paste::paste! {
            #[typed_visit(Visitor)]
            // using paste! macro, combine visit_mut_ and $ty into snakecase
            fn [<visit_mut_$ty:snake>](&mut self, n: &mut $ty) {
                crate::visit_mut_macros_helper::call_visit_mut_inner(
                    &self.visitor,
                    &self.visitor_context,
                    n,
                    &format!("visit{}", stringify!([<$ty:camel>])),
                    stringify!($ty),
                    None
                );

                n.visit_mut_children_with(self);
            }
        }
    };
}

/// Macro wraps each visit_mut_NODE when type of node is Vec<T>
#[macro_export]
macro_rules! write_visit_mut_plural {
    ($ty:ident) => {
        paste::paste! {
            #[typed_visit(Visitor)]
            fn [<visit_mut_$ty:snake s>](&mut self, n: &mut Vec<$ty>) {
                crate::visit_mut_macros_helper::call_visit_mut_inner(
                    &self.visitor,
                    &self.visitor_context,
                    n,
                    &format!("visit{}s", stringify!([<$ty:camel>])),
                    stringify!([<$ty s>]),
                    None
                );

                n.visit_mut_children_with(self);
            }
        }
    };
}

/// Macro wraps each visit_mut_NODE with path
#[macro_export]
macro_rules! write_visit_mut_path {
  ($capital: ident, $ty: ident) => {
    paste::paste! {
          #[typed_visit(PathVisitor)]
          fn [<visit_mut_$capital:lower _$ty:snake>](&mut self, n: &mut [<$capital:upper $ty>], p: &mut AstKindPath<AstParentKind>) {
            crate::visit_mut_macros_helper::call_visit_mut_inner(
                  &self.visitor,
                  &self.visitor_context,
                  n,
                  &format!("visit{}", stringify!([<$capital$ty:camel>])),
                  stringify!([<$capital:upper $ty>]),
                  Some(p)
              );

              n.visit_mut_children_with_path(self, p);
          }
      }
  };
  ($ty:ident) => {
    paste::paste! {
          #[typed_visit(PathVisitor)]
          fn [<visit_mut_$ty:snake>](&mut self, n: &mut $ty, p: &mut AstKindPath<AstParentKind>) {
            crate::visit_mut_macros_helper::call_visit_mut_inner(
                  &self.visitor,
                  &self.visitor_context,
                  n,
                  &format!("visit{}", stringify!([<$ty:camel>])),
                  stringify!($ty),
                  Some(p)
              );

              n.visit_mut_children_with_path(self, p);
          }
      }
  };
}

/// Macro wraps each visit_mut_NODE with path when type of node is Vec<T>
#[macro_export]
macro_rules! write_visit_mut_path_plural {
  ($ty:ident) => {
    paste::paste! {
          #[typed_visit(PathVisitor)]
          fn [<visit_mut_$ty:snake s>](&mut self, n: &mut Vec<$ty>, p: &mut AstKindPath<AstParentKind>) {
            crate::visit_mut_macros_helper::call_visit_mut_inner(
                  &self.visitor,
                  &self.visitor_context,
                  n,
                  &format!("visit{}s", stringify!([<$ty:camel>])),
                  stringify!([<$ty s>]),
                  Some(p)
              );

              n.visit_mut_children_with_path(self, p);
          }
      }
  };
}
