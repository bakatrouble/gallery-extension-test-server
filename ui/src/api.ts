export interface IndexItem {
    path: string;
    mime: string;
    name: string;
    is_dir: boolean;
}

export interface IndexResponse {
    items: IndexItem[];
    current_path: string;
    parent_path: string;
    root_path: string;
}

const apiUrl = import.meta.env.PROD ? '' : 'http://127.0.0.1:9474';

export const fetchIndex = async (path: string) => {
    const response = await fetch(`${apiUrl}/api/index?path=${path}`);
    return response.json() as Promise<IndexResponse>;
}

export const getThumbnailUrl = (path: string) => {
    return `${apiUrl}/api/thumbnail?path=/${path}`;
}

export const getDownloadUrl = (path: string) => {
    return `${apiUrl}/api/download?path=/${path}`;
}