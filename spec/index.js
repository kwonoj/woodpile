const { describe, it } = require('node:test');
const { visit, compat } = require('../packages/woodpile/pkg');
const { parseSync } = require('@swc/core');
const assert = require('node:assert');

describe('visit', () => {
  it('should visit plain nodes', () => {
    const ast = parseSync('console.log("Hello, World!")');

    const visits = [];
    const visitor = {
      context: 1,
      visit: {
        visitProgram: (node, self) => {
          assert(self.context === 1);
          visits.push(node);
        },
        visitExpr: (node, self) => {
          assert(self.context === 1);
          visits.push(node);
        }
      }
    };

    visit(ast, visitor);

    assert(visits.length === 5);
  });

  it('should visit plural nodes', () => {
    const ast = parseSync('console.log("Hello, World!");console.log("Hello, World!");');

    const visits = [];
    const visitor = {
      context: 1,
      visit: {
        visitModuleItems: (node, self) => {
          assert(self.context === 1);
          visits.push(node);
        },
      }
    };

    visit(ast, visitor);
    assert(visits.length === 1);
  });

  it('should visit capital nodes', () => {
    const ast = parseSync('<div>text</div>', {
      syntax: 'ecmascript',
      jsx: true,
    });

    const visits = [];
    const visitor = {
      context: 1,
      visit: {
        visitJsxElement: (node, self) => {
          assert(self.context === 1);
          visits.push(node);
        },
      }
    };

    visit(ast, visitor);
    assert(visits.length === 1);
  });
})

describe('visitWithPath', () => {
  it('should visit plain nodes', () => {
    const ast = parseSync('console.log("Hello, World!")');

    const visits = [];
    const visitor = {
      context: 1,
      visitWithPath: {
        visitProgram: (node, path, self) => {
          assert(self.context === 1);
          assert(Array.isArray(path) && path.length === 0);
          visits.push(node);
        },
        visitExpr: (node, path, self) => {
          assert(self.context === 1);
          assert(path.length > 1);
          visits.push(node);
        }
      }
    };

    visit(ast, visitor);

    assert(visits.length === 5);
  });

  it('should visit plural nodes', () => {
    const ast = parseSync('console.log("Hello, World!");console.log("Hello, World!");');

    const visits = [];
    const visitor = {
      context: 1,
      visitWithPath: {
        visitModuleItems: (node, path, self) => {
          assert(self.context === 1);
          assert(path.length === 2);
          visits.push(node);
        },
      }
    };

    visit(ast, visitor);
    assert(visits.length === 1);
  });

  it('should visit capital nodes', () => {
    const ast = parseSync('<div>text</div>', {
      syntax: 'ecmascript',
      jsx: true,
    });

    const visits = [];
    const visitor = {
      context: 1,
      visitWithPath: {
        visitJsxElement: (node, path, self) => {
          assert(self.context === 1);
          assert(path.length === 6);
          visits.push(node);
        },
      }
    };

    visit(ast, visitor);
    assert(visits.length === 1);
  });
})

describe('compat', () => {
  // naive check, don't validate if the output is sound
  it('should convert', () => {
    const ast = parseSync('console.log("Hello, World!");console.log("Hello, World!");');
    const compatAst = compat(ast);

    assert(compatAst.type === "File");
  });
});