import { ref } from "vue";
import { SizeType } from "../models/SizeType";

// Shared so the context menu rescans with the size type picked on the page.
const sizeType = ref(SizeType.Physical);

export function useSizeType() {
    return { sizeType };
}
