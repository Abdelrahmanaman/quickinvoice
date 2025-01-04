import { SignOut } from "@phosphor-icons/react/dist/ssr";
import {
  Content,
  Dropdown,
  Item,
  Separator,
  Trigger,
} from "../ui/dropdown-menu";
import {
  CaretUpDown,
  CirclesThreePlus,
  Faders,
  QuestionMark,
  User,
} from "@phosphor-icons/react";
export default function SidebarFooter() {
  return (
    <Dropdown>
      <Trigger>
        <div className="flex items-center gap-1">
          <User />
          <span>Profile</span>
        </div>
      </Trigger>
      <Content>
        <Item className="flex items-center justify-normal gap-2">
          <Faders />
          Profile Settings
        </Item>
        <Item className="flex items-center justify-normal gap-2">
          <QuestionMark />
          Help Center
        </Item>
        <Item className="flex items-center justify-normal gap-2">
          <CirclesThreePlus />
          Upgrade Plan
        </Item>
        <Separator />
        <Item>
          <span>Log out</span>
          <SignOut />
        </Item>
      </Content>
    </Dropdown>
  );
}
