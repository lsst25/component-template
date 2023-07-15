import { useMutation } from "@tanstack/react-query";
import { APIService } from "features/common";
import { queryClient } from "api/queryClient";
import { clientAPI } from "api/clientAPI";
import { NetworkError } from "features/nmp-network";
import HttpError = NetworkError.HttpError;

// TODO: change endpoint
export const endpoint = (id: $pascal_name$Id) => APIService.getUrl(`/$name$/${id}`);

export const use$pascal_name$Mutation = () => {
  return useMutation<void, HttpError, $pascal_name$Id>({
    mutationFn: async ($pascal_name$Id) => {

    },
    onSettled: async () => {

    },
  });
};
