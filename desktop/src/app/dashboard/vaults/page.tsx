"use client";
import Heading from "@/components/Heading";
import View from "@/components/View";
import { PlusIcon } from "@heroicons/react/24/solid";
import type { FormProps } from "antd";
import { Button, Form, Input, Modal } from "antd";
import { SearchProps } from "antd/es/input";
import { useState } from "react";
const { TextArea } = Input;

import { Space, Table } from "antd";

const { Column } = Table;

interface DataType {
  key: React.Key;
  name: string;
  "last modified": string;
  "created on": string;
  description: string;
}


type FormFieldTypes = {
  title?: string;
  "last modified"?: string;
};

export default function Home() {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [keywords, setKeywords] = useState<Array<string>>([]);

  const [form] = Form.useForm();
  const onSearch: SearchProps["onSearch"] = (value, _e, info) =>
    console.log(info?.source, value);

  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    console.log("Success:", { ...values, keywords });
    setIsModalOpen(false);
    form.resetFields();
  };

  const submitFormFailed: FormProps<FormFieldTypes>["onFinishFailed"] = (
    errorInfo
  ) => {
    console.log("Failed:", errorInfo);
  };

  const showModal = () => {
    setIsModalOpen(true);
  };
  const handleOk = () => {
    setIsModalOpen(false);
    form.submit();
  };
  const handleCancel = () => {
    setIsModalOpen(false);
    form.resetFields();
  };

  return (
    <>
      <View className="my-6 relative">
        <View className="flex justify-between items-center">
          <Heading>Vaults</Heading>
          <Button
            className=" w-fit bg-app-600 shadow text-white inline-flex text-sm  px-2"
            onClick={showModal}
          >
            <PlusIcon className="w-6 h-6" /> Create new{" "}
          </Button>
        </View>
      </View>
      <View>
        <Table<DataType> className="my-6">
          <Column title="Name" dataIndex="name" key="name" />
          <Column
            title="Description"
            dataIndex="description"
            key="description"
          />
          <Column title="created on" dataIndex="created on" key="created on" />
          <Column
            title="Last modified"
            dataIndex="last modified"
            key="last modified"
          />

          <Column
            title="Action"
            key="action"
            render={(_: any, record: DataType) => (
              <Space size="middle">
                <a>Invite</a>
                <a>Delete</a>
              </Space>
            )}
          />
        </Table>
      </View>

      <View className=" h-screen ">
        <Modal
          title="New Vault"
          open={isModalOpen}
          onOk={handleOk}
          centered
          onCancel={handleCancel}
          afterClose={() => submitForm}
          footer={[
            <Button key="back" onClick={handleCancel}>
              Cancel
            </Button>,
            <Button key="submit" type="primary" onClick={handleOk}>
              Submit
            </Button>,
          ]}
        >
          <Form
            initialValues={{ remember: true }}
            onFinish={submitForm}
            onFinishFailed={submitFormFailed}
            autoComplete="off"
            name="save-data"
            layout="vertical"
            className="my-4 flex flex-col gap-y-"
            form={form}
          >
            <Form.Item<FormFieldTypes>
              label="Title"
              name="title"
              rules={[{ required: true, message: "Please input the title!" }]}
            >
              <Input
                autoFocus
                type="text"
                name="title"
                className="w-full rounded py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
              />
            </Form.Item>
            <Form.Item<FormFieldTypes>
              label="Description"
              rules={[
                { required: true, message: "Please input the description" },
              ]}
            >
              <TextArea
                showCount
                maxLength={100}
                name="description"
                style={{ height: 120, resize: "none" }}
              />
            </Form.Item>
          </Form>
        </Modal>
      </View>
    </>
  );
}
