import { FunctionComponent, ReactElement, useState } from 'react';
import {
    CreateDraftGameContext,
    CreateGameProvider,
    DraftGame,
    DraftPlayer,
    DraftPublishedInfo,
} from './create-game.provider';

interface Props {
    children: ReactElement | ReactElement[];
}

export const DraftGameProvider: FunctionComponent<Props> = ({ children }) => {
    const [draftGame, setDraftGame] = useState<DraftGame | undefined>(
        undefined,
    );
    const [draftGameInfo, setDraftGameInfo] = useState<
        DraftPublishedInfo | undefined
    >(undefined);
    const [currentPlayers, setCurrentPlayers] = useState<DraftPlayer[]>([]);

    const value: CreateDraftGameContext = {
        draftGame,
        draftGameInfo,
        currentPlayers,
        createNewGame: async data => {
            console.info(data, 'provider');
            setDraftGame(data);
            return { gameId: '', gameCode: '' };
        },
        updateGameInfo: async data => {
            console.info(data);
        },
        joinTeam: async id => {
            console.info(id);
            return '';
        },
    };

    return <CreateGameProvider value={value}>{children}</CreateGameProvider>;
};
