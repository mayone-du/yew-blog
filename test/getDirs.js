import { fs } from "fs";
const dirs = fs.readdirSync("/data");
export const logDirs = () => console.log(dirs);
