import { FunctionComponent } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { byPrefixAndName } from '@awesome.me/kit-7617cb16d6/icons';
import { CreateGameForm } from './create-game-form.component';
import { GameDraftState, useCreateGameInfo } from './create-game.provider';
import { DraftGameProvider } from './draft-game.provider';
import { FormikValues } from 'formik';

export const CreateGame: FunctionComponent = () => {
    return (
        <div className='container py-2'>
            <div className='row'>
                <h2 className='text-center'>Create New Game</h2>
            </div>
            <div className='row'>
                <DraftGameProvider>
                    <GameForm />
                </DraftGameProvider>

                <div className='col-12 col-md-6'>
                    <ul className='list-group'>
                        <li className='list-group-item list-group-item-light'>
                            Current Players
                        </li>

                        <li className='list-group-item'>You</li>
                        <li className='list-group-item disabled'>
                            <FontAwesomeIcon
                                icon={byPrefixAndName.far['spinner-scale']}
                                spinPulse
                                className='me-2'
                            />
                            Waiting for players to join
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    );
};

const GameForm: FunctionComponent = () => {
    const { createNewGame } = useCreateGameInfo();
    const initialValues = {
        gameName: '',
        hostName: '',
        isPrivateGame: true,
        isTeamsAllowed: true,
        isComputerFilled: false,
    };

    const onFormSubmit = async (values: FormikValues) => {
        const _values = {
            state: GameDraftState.DraftUnpublished,
            gameName: values.gameName,
            hostName: values.hostName,
            isPrivateGame: values.isPrivateGame,
            isTeamsAllowed: values.isTeamsAllowed,
            isComputerFilled: values.isComputerFilled,
        };

        await createNewGame(_values);
    };

    return (
        <div className='col-12 col-md-6'>
            <CreateGameForm
                initialValues={initialValues}
                handleFormSubmit={onFormSubmit}
            />
        </div>
    );
};
