import { FunctionComponent, useEffect, useRef } from 'react';
import { Tooltip } from 'bootstrap';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { byPrefixAndName } from '@awesome.me/kit-7617cb16d6/icons';
import { CheckboxInput, Form, TextInput } from 'libs/ui/form';
import { FormikValues, useFormikContext } from 'formik';
import {
    DraftGame,
    GameDraftState,
    useCreateGameInfo,
} from './create-game.provider';
import { isNil } from 'lodash';

type GameFormValues = Omit<DraftGame, 'state'>;
interface Props {
    initialValues: GameFormValues;
    handleFormSubmit?: (values: FormikValues) => Promise<void>;
}

export const CreateGameForm: FunctionComponent<Props> = ({
    initialValues,
    handleFormSubmit,
}) => {
    let tooltipRef = useRef(null);
    useEffect(() => {
        let current = tooltipRef.current;
        if (current !== null) {
            new Tooltip(current);
        }
    }, [tooltipRef]);

    const validation = (values: Record<string, any>) => {
        const errors: Partial<GameFormValues> = {};

        if (!values.hostName) errors.hostName = 'Required';

        return errors;
    };

    return (
        <Form
            initialValues={initialValues}
            handleFormSubmission={handleFormSubmit ?? console.log}
            validate={validation}
        >
            <TextInput type='text' name='hostName' label='Host Name' />
            <TextInput
                type='text'
                name='gameName'
                label='Game Name'
                helpText='Will default to host name if not specified'
            />

            <CheckboxInput name='isPrivateGame' label='Private Game' isSwitch />
            <CheckboxInput name='isTeamsAllowed' label='Allow Teams' isSwitch />
            <CheckboxInput
                name='isUsingComputerFill'
                label={
                    <>
                        Fill empty slots with computer players
                        <FontAwesomeIcon
                            icon={byPrefixAndName.far['circle-question']}
                            className='ms-2'
                            size='xs'
                            data-bs-toggle='tooltip'
                            data-bs-title='For games with less than four players, or uneven teams'
                            ref={tooltipRef}
                        />
                    </>
                }
                isSwitch
                disabled
            />

            <Buttons />
        </Form>
    );
};

const Buttons: FunctionComponent = () => {
    const { submitForm } = useFormikContext();
    const { draftGame } = useCreateGameInfo();

    let buttons;
    if (isNil(draftGame)) {
        buttons = (
            <button
                className='btn btn-primary'
                onClick={() => submitForm}
                type='submit'
            >
                Create game
            </button>
        );
    } else {
        buttons = (
            <>
                <button className='btn btn-success'>Start game</button>
                <button className='btn btn-primary'>Update game info</button>
                <button className='btn btn-danger'>Delete game</button>
            </>
        );
    }

    return <div className='d-grid gap-2'>{buttons}</div>;
};
