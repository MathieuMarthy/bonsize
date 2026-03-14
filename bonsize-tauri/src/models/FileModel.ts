
export interface FileModel {
    path: string,
    is_directory: boolean,
    size: number,
    children: FileModel[],
    depth: number,
}
