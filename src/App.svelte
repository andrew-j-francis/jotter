<script lang="ts">
    import {invoke} from '@tauri-apps/api/tauri'
    import "carbon-components-svelte/css/g90.css";
    import {
        Button, Column, Content, DataTable, FileUploader,
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
    let jwtList = [];

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
        invoke('generate_jwt', {
            connectedAppConsumerKey: consumerKey,
            orgUsername: username,
            pemFileContents: fileText
        }).then((newJWT) => {
                jwtList = [...jwtList, {
                    id: newJWT,
                    create: new Date(),
                    jwt: newJWT
                }];
            }
        )
    }

    function copyToClipboard(generatedJWT) {
        invoke('copy_to_clipboard', {jwt: generatedJWT})
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
            <SideNavLink text="OCI Setup Quip" href="https://quip.com/aHwbAgtEj4NL"/>
            <SideNavLink text="Github Repo" href="https://github.com/andrew-j-francis/jotter"/>
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

        <hr>
        <h4>Generated JWTs</h4>
        <p>Note: JWTs expire after 1 hour</p>
        <DataTable
                headers={[
                        { key: "create", value: "Created" },
                        { key: "jwt", value: "JWT" },]}
                rows={jwtList}
        >
            <svelte:fragment slot="cell-header" let:header>
                {#if header.key === "jwt"}
                    JWT
                {:else}
                    {header.value}
                {/if}
            </svelte:fragment>
            <svelte:fragment slot="cell" let:row let:cell>
                {#if cell.key === "jwt"}
                    <div class="formInput">
                        <Button on:click={() => copyToClipboard(cell.value)}>
                            Copy To Clipboard
                        </Button>
                    </div>
                {:else}
                    {cell.value}
                {/if}
            </svelte:fragment>
        </DataTable>
    </Content>

</main>