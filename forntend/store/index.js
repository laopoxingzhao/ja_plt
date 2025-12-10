import {defineStore} from 'pinia'


export const useCountStore ('c',{
	state:() =>{
		return {title:"ddd"}
	},
		
	actions :{
		changetitle(t){
			this.title = t
		}
	}
})