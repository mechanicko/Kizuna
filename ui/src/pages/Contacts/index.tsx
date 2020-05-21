import { IonContent, IonFab, IonFabButton, IonIcon } from "@ionic/react";
import { add } from "ionicons/icons";
import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { withRouter } from "react-router-dom";
import { setContacts } from "../../redux/contacts/actions";
import { RootState } from "../../redux/reducers";
import ContactList from "./ContactList";
import ContactsHeader from "./ContactsHeader";
import styles from "./style.module.css";

const dump = [
  { address: "test1", username: "Neil" },
  { address: "test2", username: "Dave" },
  { address: "test3", username: "Tatsuya" },
  { address: "test4", username: "Tomato" },
  { address: "test5", username: "Potato" },
  { address: "test6", username: "Akira" },
  { address: "test7", username: "Nicko" },
  { address: "test8", username: "Zendaya" },
  { address: "test9", username: "Wakabayashi" },
  { address: "test10", username: "Sato" },
  { address: "test11", username: "Pangarungan" },
  { address: "test12", username: "Gardose" },
  { address: "test13", username: "Sasaki" },
  { username: "test14" },
  { username: "test15" },
];

const Contacts = ({ history }: { history: any }) => {
  const { indexedContacts } = useSelector((state: RootState) => state.contacts);
  const dispatch = useDispatch();
  const [search, setSearch] = useState("");

  useEffect(() => {
    dispatch(setContacts(dump));
  }, []);
  return (
    <>
      <ContactsHeader search={search} setSearch={setSearch} />
      <IonContent>
        <ContactList search={search} indexedContacts={indexedContacts} />
        <IonFab
          onClick={() => history.push("/add")}
          className={styles.fab}
          vertical="bottom"
          horizontal="end"
          slot="fixed"
        >
          <IonFabButton>
            <IonIcon icon={add} />
          </IonFabButton>
        </IonFab>
      </IonContent>
    </>
  );
};

export default withRouter(Contacts);
