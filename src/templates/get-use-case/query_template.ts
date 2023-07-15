import { useQuery } from "@tanstack/react-query";
import { APIService } from "features/common";
import { clientAPI } from "api/clientAPI";
import { NetworkError } from "features/nmp-network";
import HttpError = NetworkError.HttpError;
import { $pascal_name$QueryKeys } from "./$pascal_nam$-query-keys";
import { $paskal_name$DtoType } from "../../dtos/$name$-dto.type";
import { $paskal_name$Model } from "../../models/$name$.model";

// TODO: change endpoint
const endpoint = APIService.getUrl("/$name$");

export const use$paskal_name$Query = () => {
  return useQuery<$paskal_name$Model[], HttpError>({
    queryKey: $paskal_name$QueryKeys.all(),
    queryFn: async () => {
      const response = await clientAPI.get<$paskal_name$DtoType[]>(endpoint);

      return response.map($paskal_name$Model.create);
    },
  });
};
