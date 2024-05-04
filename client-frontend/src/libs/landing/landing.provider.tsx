import {
    createContext,
    FunctionComponent,
    ReactElement,
    useContext,
    useEffect,
    useState,
} from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Fetcher } from 'swr';

export interface PublicLobbyGame {
    game_code: string;
    game_name: string;
    player_count: number;
}

interface Context {
    fetchPublicLobbies: Fetcher<PublicLobbyGame[], string>;
}

const context = createContext<Context | undefined>(undefined);
export const LandingProvider: FunctionComponent<{ children: ReactElement }> = ({
    children,
}) => {
    const value = {
        fetchPublicLobbies: () =>
            invoke<PublicLobbyGame[]>('list_public_draft_games'),
    };

    return <context.Provider value={value}>{children}</context.Provider>;
};

export const useLobbyProvider = (): Context => {
    let _context = useContext(context);
    if (!_context)
        throw new Error(
            'useLobbyProvider must exist inside of LandingProvider',
        );

    return _context;
};
