import {createContext, useContext} from 'solid-js';
import {ToasterStore} from 'terracotta';
import {notifications, Notification} from '../stores/notifications';

const Notiifications = createContext<ToasterStore<Notification>>(notifications);

export const NotificationsProvider = Notiifications.Provider;

export const useNotifications = () => {
  return useContext(Notiifications);
};
