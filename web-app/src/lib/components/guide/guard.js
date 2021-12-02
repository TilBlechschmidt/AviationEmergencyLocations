import { goto } from '$app/navigation';
import { page } from '$app/stores';
import { browser } from '$app/env';
import { passedDisclaimers } from '$lib/stores';

const routeStorageKey = "preDisclaimerDestination";

export const DISCLAIMERS = {
    INTRODUCTION: {
        key: "intro",
        route: "/guide",
    },
    SAFETY_GUIDE: {
        key: "safety",
        route: "/guide/finalWords"
    }
}
export const INTRODUCTION = "intro";
export const SAFETY_GUIDE = "safety";

export const completeDisclaimer = disclaimer => {
    passedDisclaimers.update(value => value.concat([disclaimer.key]));
};

export const requireDisclaimer = disclaimers => {
    if (!browser) return;

    let path;
    let passed;

    page.subscribe(p => path = p.path);
    passedDisclaimers.subscribe(d => passed = d);

    for (const disclaimer of disclaimers) {
        if (passed.indexOf(disclaimer.key) == -1) {
            if (!sessionStorage.getItem(routeStorageKey)) sessionStorage.setItem(routeStorageKey, path)
            goto(disclaimer.route);
        }
    }
};

export const hasUserPassedDisclaimer = disclaimer => {
    let passed;
    passedDisclaimers.subscribe(d => passed = d);
    return passed.indexOf(disclaimer.key) > -1;
}

export const restorePreDisclaimerDestination = fallback => {
    if (!browser) return;

    const preDisclaimerDestination = sessionStorage.getItem(routeStorageKey) || fallback;
    sessionStorage.removeItem(routeStorageKey);
    goto(preDisclaimerDestination);
}
