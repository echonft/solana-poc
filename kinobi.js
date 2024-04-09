const { createFromIdls, renderJavaScriptVisitor, updateProgramsVisitor, setAnchorDiscriminatorsVisitor } = require(
  "@metaplex-foundation/kinobi",
);
const { join } = require("path");

// Instantiate Kinobi.
const kinobi = createFromIdls([
  join(__dirname, "target", "idl", "echo.json"),
]);

// Update the Kinobi tree using visitors...
kinobi.update(
  updateProgramsVisitor({
    "echo": {
      publicKey: "Gzq13nAmkDZMgFjKs8Zd6jJbXE2X6iJJ4FEe2BqWcVWM",
      origin: "anchor",
    },
  }),
);
kinobi.update(
  setAnchorDiscriminatorsVisitor(),
);

// Render JavaScript.
const jsDir = join(__dirname, "target", "sdk");
kinobi.accept(renderJavaScriptVisitor(jsDir));
