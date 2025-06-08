import fs from "node:fs";
import { rootNodeFromAnchor } from "@codama/nodes-from-anchor";
import {
  createFromRoot,
  updateInstructionsVisitor,
  type ProgramUpdates,
} from "codama";
import anchorIdl from "../programs/holosim/SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF-idl.json";

const codama = createFromRoot(rootNodeFromAnchor(anchorIdl));

// TODO: will need to revisit this later...arg errors with codama renderer
const map: Record<string, ProgramUpdates> = {
  closeCraftingProcess: { delete: true },
  scanForSurveyDataUnits: { delete: true },
  stopMiningAsteroid: { delete: true },
};

codama.update(updateInstructionsVisitor(map));

fs.writeFileSync("./programs/holosim/codamaIDL.json", codama.getJson());
