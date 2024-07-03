import { describe, it } from "node:test";
import assert from "node:assert";
import { parseSync, printSync } from "@swc/core";
import { visit, compat } from "../packages/woodpile/pkg";

describe('visit', () => {
  it('should visit plain nodes', () => {
    const ast = parseSync('console.log("Hello, World!")');

    const visits: Array<any> = [];
    const visitor = {
      context: 1,
      visit: {
        visitProgram: (node: any, self: any) => {
          assert(self.context === 1);
          visits.push(node);
        },
        visitExpr: (node: any, self: any) => {
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

    const visits: Array<any> = [];
    const visitor = {
      context: 1,
      visit: {
        visitModuleItems: (node: any, self: any) => {
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

    const visits: Array<any> = [];
    const visitor = {
      context: 1,
      visit: {
        visitJsxElement: (node: any, self: any) => {
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

    const visits: Array<any> = [];
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

    const visits: Array<any> = [];
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

    const visits: Array<any> = [];
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

describe("mutate", () => {
  it("should be able to mutate", () => {
    const ast = parseSync('console.log("TEST")');

    const visitor = {
      context: 1,
      visit: {
        visitExpr: (node: any, self: any) => {
          if (node.type === "StringLiteral" && node.value === "TEST") {
            const newNode = {
              ...node,
              value: "CHANGED",
              raw: "CHANGED"
            };
            return newNode;
          }
        }
      }
    };

    const ret = visit(ast, visitor);
    const { code: updated } = printSync(ret);
    assert(updated.trim() === `console.log("CHANGED");`);
  });
});