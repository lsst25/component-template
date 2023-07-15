import { useQuery } from "@tanstack/react-query";
import { APIService } from "features/common";
import { clientAPI } from "api/clientAPI";
import { NetworkError } from "features/nmp-network";
import HttpError = NetworkError.HttpError;
import { $pascal_name$QueryKeys } from "./$name$-query-keys";
import { $pascal_name$DtoType } from "../../dtos/$name$-dto.type";
import { $pascal_name$Model } from "../../models/$name$.model";

// TODO: change endpoint
const endpoint = APIService.getUrl("/$name$");

export const use$pascal_name$Query = () => {
  return useQuery<$pascal_name$Model[], HttpError>({
    queryKey: $pascal_name$QueryKeys.all(),
    queryFn: async () => {
      const response = await clientAPI.get<$pascal_name$DtoType[]>(endpoint);

      return response.map($pascal_name$Model.create);
    },
  });
};
