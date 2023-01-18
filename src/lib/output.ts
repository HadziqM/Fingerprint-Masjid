import { writable } from "svelte/store";
import type {Output} from "../type"

let zero:Output = {machine:[],start:"",finish:""
  ,status:400,data:0,valid:0,invalid:0,double:0};
export let output = writable(zero)
