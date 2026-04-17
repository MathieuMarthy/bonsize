import { FileModel } from "../models/FileModel";

export function attachParents(file: FileModel, parent?: FileModel) {
    file.parent = parent;
    file.children?.forEach((child) => attachParents(child, file));
}

export function updateParentSizes(parent: FileModel, fileSize: number) {
    let currentParent: FileModel | null = parent;
    while (currentParent) {
        currentParent.size += fileSize;
        currentParent = currentParent.parent || null;
    }
}
