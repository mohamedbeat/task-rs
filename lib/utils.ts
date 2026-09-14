export { cn } from "cn"
export function getMemoryColor(bytes: number) {
    const MB = 1024 * 1024;
    const GB = 1024 * MB;

    if (bytes < 200 * MB) {
        return 'bg-transparent';
    } else if (bytes < 1 * GB) {
        return 'bg-[#fff3b0] text-black'; // yellow
    } else if (bytes < 5 * GB) {
        return 'bg-[#ffcc80] text-black'; // orange
    } else {
        return 'bg-[#ff8a80] text-black'; // red
    }
}

export function getCpuColor(percentage: number) {

    if (percentage < 10) {
        return 'bg-transparent';
    } else if (percentage < 25) {
        return 'bg-[#fff3b0] text-black'; // yellow
    } else if (percentage < 50) {
        return 'bg-[#ffcc80] text-black'; // orange
    } else {
        return 'bg-[#ff8a80] text-black'; // red
    }
}