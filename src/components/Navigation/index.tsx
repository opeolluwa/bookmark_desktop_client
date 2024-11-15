"use client";
import { AppstoreOutlined } from "@ant-design/icons";
import {
  ArrowLeftStartOnRectangleIcon,
  BellIcon,
  Cog6ToothIcon,
  QueueListIcon,
  RectangleStackIcon,
  UserIcon,
} from "@heroicons/react/24/outline";
import { ItemType, MenuItemType } from "antd/es/menu/interface";
import Link from "next/link";

export const routes: ItemType<MenuItemType>[] = [
  {
    icon: <AppstoreOutlined className="w-6 h-6" style={{ fontSize: "24px" }} />,
    label: <Link href={"/dashboard"}>Dashboard</Link>,
    key: "dashboard",
  },
  {
    icon: <RectangleStackIcon className="w-6 h-6" />,
    label: <Link href={"/dashboard/collections"}>collections</Link>,
    key: "collections",
  },

  {
    icon: <BellIcon className="w-6 h-6" />,
    label: <Link href={"/dashboard/notification"}>Notification</Link>,
    key: "notification",
  },
  {
    icon: <UserIcon className="w-6 h-6" />,
    label: <Link href={"/dashboard/profile"}>Profile</Link>,
    key: "profile",
  },
  {
    icon: <Cog6ToothIcon className="w-6 h-6" />,
    label: <Link href={"/dashboard/settings"}>Settings</Link>,
    key: "settings",
  },

  {
    icon: <QueueListIcon className="w-6 h-6" />,
    label: <Link href={"/dashboard/activity"}>Activity</Link>,
    key: "activity",
  },
  {
    icon: <ArrowLeftStartOnRectangleIcon className="w-6 h-6" />,
    label: <Link href={"/"}>log out</Link>,
    key: "logout",
  },
];
