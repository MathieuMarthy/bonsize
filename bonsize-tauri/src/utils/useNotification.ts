import { ref } from "vue";

export interface Notification {
    id: number;
    message: string;
    type: "success" | "error" | "info";
}

const notifications = ref<Notification[]>([]);
let nextId = 1;

export function useNotification() {
    function notify(message: string, type: "success" | "error" | "info" = "info", duration = 3000) {
        const id = nextId++;
        notifications.value.push({ id, message, type });

        setTimeout(() => {
            removeNotification(id);
        }, duration);
    }

    function removeNotification(id: number) {
        notifications.value = notifications.value.filter(n => n.id !== id);
    }

    return {
        notifications,
        notify,
        removeNotification
    };
}