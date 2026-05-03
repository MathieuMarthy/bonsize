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

export function flattenFiles(file: FileModel, result: FileModel[] = []) {
    result.push(file);

    if (file.children) {
        for (const child of file.children) {
            flattenFiles(child, result);
        }
    }
    return result;
}
