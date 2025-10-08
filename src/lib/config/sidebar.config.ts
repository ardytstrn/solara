import type { ActivePage } from "$lib/types";
import UserIcon from "$lib/icons/UserIcon.svelte";
import PerformanceIcon from "$lib/icons/PerformanceIcon.svelte";
import AddInstanceIcon from "$lib/icons/AddInstanceIcon.svelte";
import ModsIcon from "$lib/icons/ModsIcon.svelte";
import InstancesIcon from "$lib/icons/InstancesIcon.svelte";
import SettingsIcon from "$lib/icons/SettingsIcon.svelte";
import type {Component} from "svelte";

export interface SidebarItem {
    id: ActivePage | 'add-instance';
    label: string;
    icon: Component;
    isAction?: boolean;
}

export const sidebarItems: SidebarItem[][][] = [
    [
        [
            { id: 'accounts', label: 'Accounts', icon: UserIcon },
        ],
        [
            { id: 'instances', label: 'Instances', icon: InstancesIcon },
            { id: 'mods', label: 'Mods', icon: ModsIcon },
            { id: 'performance', label: 'Performance', icon: PerformanceIcon },
        ],
        [
            { id: 'add-instance', label: 'Add Instance', icon: AddInstanceIcon, isAction: true }
        ]
    ],
    [
        [
            { id: 'settings', label: 'Settings', icon: SettingsIcon },
        ]
    ],
];
