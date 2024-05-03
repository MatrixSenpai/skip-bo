import { createContext, useContext } from 'react';

export enum GameDraftState {
    DraftUnpublished,
    DraftPublished,
}
export interface DraftGame {
    state: GameDraftState;
    hostName: string;
    gameName: string | undefined;
    isPrivateGame: boolean;
    isTeamsAllowed: boolean;
    isComputerFilled: boolean;
}
export interface DraftPublishedInfo {
    gameId: string;
    gameCode: string;
}
export interface DraftPlayer {
    id: string;
    name: string;
    teamId: string | undefined;
}

export interface CreateDraftGameContext {
    draftGame: DraftGame | undefined;
    draftGameInfo: DraftPublishedInfo | undefined;
    currentPlayers: DraftPlayer[];
    createNewGame: (data: DraftGame) => Promise<DraftPublishedInfo>;
    updateGameInfo: (data: Partial<DraftGame>) => Promise<void>;
    joinTeam: (playerId: string) => Promise<string>;
}

const context = createContext<CreateDraftGameContext | null>(null);
export const CreateGameProvider = context.Provider;
export const useCreateGameInfo = (): CreateDraftGameContext => {
    const _context = useContext(context);
    if (!_context)
        throw new Error(
            'useCreateGameInfo must be called within CreateGameProvider',
        );

    return _context;
};
