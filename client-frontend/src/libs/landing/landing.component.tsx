import { FunctionComponent } from 'react';
import { Link } from 'react-router-dom';
import { LandingProvider, useLobbyProvider } from './landing.provider';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { byPrefixAndName } from '@awesome.me/kit-7617cb16d6/icons';
import useSwr from 'swr';

export const Landing: FunctionComponent = () => {
    return (
        <div className='container py-2'>
            <div className='row mb-2'>
                <div className='col-12 text-center'>
                    <h1>Skip-Bo</h1>
                </div>
            </div>

            <div className='row'>
                <div className='col-12 col-lg-6'>
                    <div className='card'>
                        <div className='card-body'>
                            <div className='nav flex-column gap-2'>
                                <Link
                                    to='/create'
                                    className='btn btn-block btn-primary'
                                >
                                    Create new game
                                </Link>
                                <button
                                    disabled
                                    className='btn btn-block btn-disabled'
                                >
                                    Join Private Game
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                <LandingProvider>
                    <LandingLobbyList />
                </LandingProvider>
            </div>
        </div>
    );
};

const LandingLobbyList: FunctionComponent = () => {
    const { fetchPublicLobbies } = useLobbyProvider();
    const { data, mutate } = useSwr('NOVALUE', fetchPublicLobbies);

    return (
        <div className='col-12 col-lg-6'>
            <ul className='list-group'>
                <li className='list-group-item list-group-item-light d-flex justify-content-between align-items-center'>
                    <div className='ms-2 me-auto'>Public Lobbies</div>
                    <button type='button' onClick={() => mutate()}>
                        <FontAwesomeIcon icon={byPrefixAndName.far['rotate']} />
                    </button>
                </li>
                {data &&
                    data.map(game => (
                        <li
                            className='list-group-item d-flex justify-content-between align-items-center'
                            key={game.game_code}
                        >
                            <div className='ms-2 me-auto'>{game.game_name}</div>
                            <span className='badge text-bg-primary rounded-pill'>
                                {game.player_count}
                            </span>
                        </li>
                    ))}
            </ul>
        </div>
    );
};
