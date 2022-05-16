<script lang="ts">
    import {invoke} from '@tauri-apps/api/tauri'
    import "carbon-components-svelte/css/g90.css";
    import {
        Button, Column, Content, FileUploader,
        Form,
        FormGroup, Grid,
        Header, Row,
        SideNav, SideNavItems, SideNavLink,
        TextInput
    } from "carbon-components-svelte";

    let isSideNavOpen = false;
    let consumerKey;
    let username;
    let files = [];
    let fileText;

    function handleSubmit() {
        if (files[0]) {
            let fileReader = new FileReader();

            fileReader.onload = function (event) {
                fileText = event.target.result;
                console.log('File content:' + fileText);

                generateJWT();
            }

            fileReader.readAsText(files[0]);
        }
    }

    function generateJWT() {
        invoke('generate_jwt', {connectedAppConsumerKey: consumerKey, orgUsername: username, pemFileContents: fileText})
    }
</script>

<style>
    .formInput {
        margin: 1.5rem;
    }
</style>

<main>
    <Header platformName="Jotter - The JWT Generator for Salesforce OCI" bind:isSideNavOpen></Header>
    <SideNav bind:isOpen={isSideNavOpen}>
        <SideNavItems>
            <SideNavLink text="Help Quip"/>
            <SideNavLink text="Github Repo"/>
        </SideNavItems>
    </SideNav>

    <Content>
        <Grid>
            <Row>
                <Column>
                    <Form>
                        <FormGroup>
                            <div class="formInput">
                                <TextInput labelText="Connected App Consumer Key"
                                           helperText="Consumer Key related to your Connected App"
                                           bind:value={consumerKey}></TextInput>
                            </div>
                            <div class="formInput">
                                <TextInput labelText="Org Username"
                                           placeholder="john.smith@universal-containers.com"
                                           bind:value={username}></TextInput>
                            </div>
                            <div class="formInput">
                                <FileUploader buttonLabel="Upload PEM File" bind:files status="complete"></FileUploader>
                            </div>
                        </FormGroup>
                        <div class="formInput">
                            <Button on:click={handleSubmit}>Generate JWT</Button>
                        </div>
                    </Form>
                </Column>
            </Row>
        </Grid>
    </Content>

    <hr>

</main>