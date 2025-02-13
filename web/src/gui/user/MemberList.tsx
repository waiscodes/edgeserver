import { FC } from 'react';

import { useTeamMembers } from '@/api/team';
import { Button } from '@/components/button';

import { TeamInviteCreateModal } from '../invite/TeamInviteCreateModal';
import { UserPreview } from './UserPreview';

export const MemberList: FC<{ team_id: string }> = ({ team_id }) => {
    const { data: members } = useTeamMembers(team_id);

    return (
        <div className="space-y-2">
            <div className="flex items-center justify-between">
                <h2 className="h2">Members</h2>
                <TeamInviteCreateModal team_id={team_id}>
                    <Button>Invite Member</Button>
                </TeamInviteCreateModal>
            </div>
            {members && (
                <ul className="divide-y">
                    {members.map((member) => (
                        <li key={member.user_id}>
                            <UserPreview user={member} />
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
};
