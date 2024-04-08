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
      publicKey: "9YimkcCy3hXuMkCRU2CHbWGZTpKF4o4zLCkpSpGANfDN",
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
