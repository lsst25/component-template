import { useMutation } from "@tanstack/react-query";
import { APIService } from "features/common";
import { queryClient } from "api/queryClient";
import { clientAPI } from "api/clientAPI";
import { NetworkError } from "features/nmp-network";
import HttpError = NetworkError.HttpError;

// TODO: change endpoint
export const endpoint = (id: FooBarId) => APIService.getUrl(`/foo-bar/${id}`);

export const useFooBarMutation = () => {
  return useMutation<void, HttpError, FooBarId>({
    mutationFn: async (FooBarId) => {

    },
    onSettled: async () => {

    },
  });
};
