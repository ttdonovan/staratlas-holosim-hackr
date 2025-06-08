import { renderRustVisitor } from "@codama/renderers";
import { createFromRoot } from "codama";
import codamaIDL from "../programs/holosim/codamaIDL.json";

const codama = createFromRoot(codamaIDL);

codama.accept(renderRustVisitor("./programs/holosim/src/generated", {}));
