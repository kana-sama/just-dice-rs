#!/usr/bin/env node

import fs from "fs/promises";

const positions = JSON.parse(await fs.readFile("positions.json"));
const buffer = new Uint16Array(positions.flat().flatMap(x => [x.x, x.y, x.angle]));
await fs.writeFile("positions.bin", buffer);
