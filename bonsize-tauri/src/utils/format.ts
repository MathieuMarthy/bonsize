
const UNITS = ["B", "KB", "MB", "GB", "TB"];

export function formatFileSize(fileSize: number): string {
    let unitIndex = 0;

    while (fileSize >= 1024 && unitIndex < UNITS.length - 1) {
        fileSize /= 1024;
        unitIndex++;
    }

    return `${fileSize.toFixed(2)} ${UNITS[unitIndex]}`;
};
