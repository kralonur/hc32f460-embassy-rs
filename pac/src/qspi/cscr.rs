#[doc = "Register `CSCR` reader"]
pub type R = crate::R<CscrSpec>;
#[doc = "Register `CSCR` writer"]
pub type W = crate::W<CscrSpec>;
#[doc = "Field `SSHW` reader - desc SSHW"]
pub type SshwR = crate::FieldReader;
#[doc = "Field `SSHW` writer - desc SSHW"]
pub type SshwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SSNW` reader - desc SSNW"]
pub type SsnwR = crate::FieldReader;
#[doc = "Field `SSNW` writer - desc SSNW"]
pub type SsnwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - desc SSHW"]
    #[inline(always)]
    pub fn sshw(&self) -> SshwR {
        SshwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - desc SSNW"]
    #[inline(always)]
    pub fn ssnw(&self) -> SsnwR {
        SsnwR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc SSHW"]
    #[inline(always)]
    pub fn sshw(&mut self) -> SshwW<'_, CscrSpec> {
        SshwW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc SSNW"]
    #[inline(always)]
    pub fn ssnw(&mut self) -> SsnwW<'_, CscrSpec> {
        SsnwW::new(self, 4)
    }
}
#[doc = "desc CSCR\n\nYou can [`read`](crate::Reg::read) this register and get [`cscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscrSpec;
impl crate::RegisterSpec for CscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cscr::R`](R) reader structure"]
impl crate::Readable for CscrSpec {}
#[doc = "`write(|w| ..)` method takes [`cscr::W`](W) writer structure"]
impl crate::Writable for CscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCR to value 0x0f"]
impl crate::Resettable for CscrSpec {
    const RESET_VALUE: u32 = 0x0f;
}
